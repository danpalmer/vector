use crate::{
    buffers::Acker,
    event::Metric,
    sinks::util::{
        http::{Error as HttpError, HttpRetryLogic, HttpService, Response as HttpResponse},
        BatchConfig, MetricBuffer, SinkExt, TowerRequestConfig,
    },
    topology::config::{DataType, SinkConfig, SinkDescription},
    tower_request_config,
};
use chrono::{DateTime, Utc};
use futures::{Future, Poll};
use http::{uri::InvalidUri, Method, StatusCode, Uri};
use hyper;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use tower::Service;

#[derive(Debug, Snafu)]
enum BuildError {
    #[snafu(display("Invalid host {:?}: {:?}", host, source))]
    InvalidHost { host: String, source: InvalidUri },
}

#[derive(Clone)]
struct DatadogState {
    last_sent_timestamp: i64,
}

#[derive(Clone)]
struct DatadogSvc {
    config: DatadogConfig,
    state: DatadogState,
    inner: HttpService,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct DatadogConfig {
    pub namespace: String,
    #[serde(default = "default_host")]
    pub host: String,
    pub api_key: String,
    #[serde(default, flatten)]
    pub batch: BatchConfig,
    #[serde(default, flatten)]
    pub request: DatadogRequestConfig,
}

tower_request_config! {
    DatadogRequestConfig;
    in_flight_limit = 5,
    timeout = 60,
    rate_limit_duration = 1,
    rate_limit_num = 5,
    retry_attempts = 5,
    retry_backoff = 1,
}

pub fn default_host() -> String {
    String::from("https://api.datadoghq.com")
}

// https://docs.datadoghq.com/api/?lang=bash#post-timeseries-points
#[derive(Debug, Clone, PartialEq, Serialize)]
struct DatadogRequest {
    series: Vec<DatadogMetric>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct DatadogMetric {
    metric: String,
    r#type: DatadogMetricType,
    interval: Option<i64>,
    points: Vec<DatadogPoint>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DatadogMetricType {
    Gauge,
    Count,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct DatadogPoint(i64, f64);

inventory::submit! {
    SinkDescription::new::<DatadogConfig>("datadog")
}

#[typetag::serde(name = "datadog")]
impl SinkConfig for DatadogConfig {
    fn build(&self, acker: Acker) -> crate::Result<(super::RouterSink, super::Healthcheck)> {
        let sink = DatadogSvc::new(self.clone(), acker)?;
        let healthcheck = DatadogSvc::healthcheck(self.clone())?;
        Ok((sink, healthcheck))
    }

    fn input_type(&self) -> DataType {
        DataType::Metric
    }

    fn sink_type(&self) -> &'static str {
        "datadog"
    }
}

impl DatadogSvc {
    pub fn new(config: DatadogConfig, acker: Acker) -> crate::Result<super::RouterSink> {
        let batch = config.batch.unwrap_or(20, 1);

        let uri = format!("{}/api/v1/series?api_key={}", config.host, config.api_key)
            .parse::<Uri>()
            .context(super::UriParseError)?;
        let request = config.request.clone();

        let http_service = HttpService::new(move |body: Vec<u8>| {
            let mut builder = hyper::Request::builder();
            builder.method(Method::POST);
            builder.uri(uri.clone());

            builder.header("Content-Type", "application/json");
            builder.body(body).unwrap()
        });

        let datadog_http_service = DatadogSvc {
            config,
            state: DatadogState {
                last_sent_timestamp: Utc::now().timestamp(),
            },
            inner: http_service,
        };

        let sink = request
            .batch_sink(HttpRetryLogic, datadog_http_service, acker)
            .batched_with_min(MetricBuffer::new(), &batch);

        Ok(Box::new(sink))
    }

    fn healthcheck(config: DatadogConfig) -> crate::Result<super::Healthcheck> {
        let uri = format!("{}/api/v1/validate?api_key={}", config.host, config.api_key)
            .parse::<Uri>()
            .context(super::UriParseError)?;

        let request = hyper::Request::get(uri).body(hyper::Body::empty()).unwrap();

        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = hyper::Client::builder().build(https);

        let healthcheck = client
            .request(request)
            .map_err(|err| err.into())
            .and_then(|response| match response.status() {
                StatusCode::OK => Ok(()),
                other => Err(super::HealthcheckError::UnexpectedStatus { status: other }.into()),
            });

        Ok(Box::new(healthcheck))
    }
}

impl Service<Vec<Metric>> for DatadogSvc {
    type Response = HttpResponse;
    type Error = HttpError;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error> + Send + 'static>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.inner.poll_ready()
    }

    fn call(&mut self, items: Vec<Metric>) -> Self::Future {
        let now = Utc::now().timestamp();
        let interval = now - self.state.last_sent_timestamp;
        self.state.last_sent_timestamp = now;

        let input = encode_events(items, interval, &self.config.namespace);
        let body = serde_json::to_vec(&input).unwrap();

        self.inner.call(body)
    }
}

fn encode_tags(tags: HashMap<String, String>) -> Vec<String> {
    let mut pairs: Vec<_> = tags
        .iter()
        .map(|(name, value)| format!("{}:{}", name, value))
        .collect();
    pairs.sort();
    pairs
}

fn encode_timestamp(timestamp: Option<DateTime<Utc>>) -> i64 {
    if let Some(ts) = timestamp {
        ts.timestamp()
    } else {
        Utc::now().timestamp()
    }
}

fn encode_namespace(namespace: &str, name: String) -> String {
    if !namespace.is_empty() {
        format!("{}.{}", namespace, name)
    } else {
        name
    }
}

fn encode_events(events: Vec<Metric>, interval: i64, namespace: &str) -> DatadogRequest {
    let series: Vec<_> = events
        .into_iter()
        .filter_map(|event| match event {
            Metric::Counter {
                name,
                val,
                timestamp,
                tags,
            } => Some(DatadogMetric {
                metric: encode_namespace(namespace, name),
                r#type: DatadogMetricType::Count,
                interval: Some(interval),
                points: vec![DatadogPoint(encode_timestamp(timestamp), val)],
                tags: tags.map(encode_tags),
            }),
            Metric::Gauge {
                name,
                val,
                direction: None,
                timestamp,
                tags,
            } => Some(DatadogMetric {
                metric: encode_namespace(namespace, name),
                r#type: DatadogMetricType::Gauge,
                interval: None,
                points: vec![DatadogPoint(encode_timestamp(timestamp), val)],
                tags: tags.map(encode_tags),
            }),
            Metric::Histogram {
                name,
                val,
                sample_rate,
                timestamp,
                tags,
            } => {
                let mut points = Vec::new();
                for _ in 0..sample_rate {
                    let point = DatadogPoint(encode_timestamp(timestamp), val);
                    points.push(point);
                }
                Some(DatadogMetric {
                    metric: encode_namespace(namespace, name),
                    r#type: DatadogMetricType::Count,
                    interval: Some(interval),
                    points,
                    tags: tags.map(encode_tags),
                })
            }
            _ => None,
        })
        .collect();

    DatadogRequest { series }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::metric::Metric;
    use chrono::offset::TimeZone;
    use pretty_assertions::assert_eq;

    fn ts() -> DateTime<Utc> {
        Utc.ymd(2018, 11, 14).and_hms_nano(8, 9, 10, 11)
    }

    fn tags() -> HashMap<String, String> {
        vec![
            ("normal_tag".to_owned(), "value".to_owned()),
            ("true_tag".to_owned(), "true".to_owned()),
            ("empty_tag".to_owned(), "".to_owned()),
        ]
        .into_iter()
        .collect()
    }

    #[test]
    fn test_encode_tags() {
        assert_eq!(
            encode_tags(tags()),
            vec!["empty_tag:", "normal_tag:value", "true_tag:true"]
        );
    }

    #[test]
    fn test_encode_timestamp() {
        assert_eq!(encode_timestamp(None), Utc::now().timestamp());
        assert_eq!(encode_timestamp(Some(ts())), 1542182950);
    }

    #[test]
    fn encode_counter() {
        let now = Utc::now().timestamp();
        let interval = 60;
        let events = vec![
            Metric::Counter {
                name: "total".into(),
                val: 1.5,
                timestamp: None,
                tags: None,
            },
            Metric::Counter {
                name: "check".into(),
                val: 1.0,
                timestamp: Some(ts()),
                tags: Some(tags()),
            },
        ];
        let input = encode_events(events, interval, "ns");
        let json = serde_json::to_string(&input).unwrap();

        assert_eq!(
            json,
            format!("{{\"series\":[{{\"metric\":\"ns.total\",\"type\":\"count\",\"interval\":60,\"points\":[[{},1.5]],\"tags\":null}},{{\"metric\":\"ns.check\",\"type\":\"count\",\"interval\":60,\"points\":[[1542182950,1.0]],\"tags\":[\"empty_tag:\",\"normal_tag:value\",\"true_tag:true\"]}}]}}", now)
        );
    }

    #[test]
    fn encode_gauge() {
        let events = vec![Metric::Gauge {
            name: "volume".into(),
            val: -1.1,
            direction: None,
            timestamp: Some(ts()),
            tags: None,
        }];
        let input = encode_events(events, 60, "");
        let json = serde_json::to_string(&input).unwrap();

        assert_eq!(
            json,
            r#"{"series":[{"metric":"volume","type":"gauge","interval":null,"points":[[1542182950,-1.1]],"tags":null}]}"#
        );
    }

    #[test]
    fn encode_histogram() {
        let events = vec![Metric::Histogram {
            name: "login".into(),
            val: 1.0,
            sample_rate: 2,
            timestamp: Some(ts()),
            tags: None,
        }];
        let input = encode_events(events, 60, "");
        let json = serde_json::to_string(&input).unwrap();

        assert_eq!(
            json,
            r#"{"series":[{"metric":"login","type":"count","interval":60,"points":[[1542182950,1.0],[1542182950,1.0]],"tags":null}]}"#
        );
    }
}
