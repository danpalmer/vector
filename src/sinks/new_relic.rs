use crate::{
    buffers::Acker,
    sinks::http::{Encoding, HttpMethod, HttpSinkConfig},
    sinks::util::BatchConfig,
    topology::config::{DataType, SinkConfig, SinkDescription},
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Clone, Derivative)]
#[serde(rename_all = "snake_case")]
#[derivative(Default)]
pub enum NewRelicRegion {
    #[derivative(Default)]
    Us,
    Eu,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NewRelicConfig {
    pub license_key: Option<String>,
    pub insert_key: Option<String>,
    pub region: Option<NewRelicRegion>,
    #[serde(default, flatten)]
    pub batch: BatchConfig,
}

inventory::submit! {
    SinkDescription::new::<NewRelicConfig>("new_relic")
}

#[typetag::serde(name = "new_relic")]
impl SinkConfig for NewRelicConfig {
    fn build(&self, acker: Acker) -> crate::Result<(super::RouterSink, super::Healthcheck)> {
        let mut headers: IndexMap<String, String> = IndexMap::new();
        headers.insert("Content-Type".to_owned(), "application/json".to_owned());

        if let Some(license_key) = &self.license_key {
            headers.insert("X-License-Key".to_owned(), license_key.clone());
        } else if let Some(insert_key) = &self.insert_key {
            headers.insert("X-Insert-Key".to_owned(), insert_key.clone());
        } else {
            return Err(format!("must provide either 'license_key' or 'insert_key'").into());
        }

        let uri = match self.region.as_ref().unwrap_or(&NewRelicRegion::Us) {
            NewRelicRegion::Us => "https://log-api.newrelic.com/log/v1",
            NewRelicRegion::Eu => "https://log-api.eu.newrelic.com/log/v1",
        };

        let mut http_conf = HttpSinkConfig::default();
        http_conf.method = Some(HttpMethod::Post);
        http_conf.headers = Some(headers);
        http_conf.uri = uri.to_owned();
        http_conf.encoding = Encoding::Json;
        http_conf.batch = self.batch.clone();

        http_conf.build(acker)
    }

    fn input_type(&self) -> DataType {
        DataType::Log
    }

    fn sink_type(&self) -> &'static str {
        "new_relic"
    }
}
