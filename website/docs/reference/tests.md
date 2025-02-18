---
title: Unit Tests
description: Unit test configuration options
status: beta
---

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     website/docs/reference/tests.md.erb
-->

It's possible to define unit tests within a Vector configuration file that cover
a network of transforms within the topology. The intention of these tests is to
improve the maintainability of configs containing larger and more complex
combinations of transforms.

Executing tests within a config file can be done with the `test` subcommand:

```bash
vector test /etc/vector/*.toml
```

## Configuration

import Tabs from '@theme/Tabs';

<Tabs
  block={true}
  defaultValue="common"
  values={[
    { label: 'Common', value: 'common', },
    { label: 'Advanced', value: 'advanced', },
  ]
}>

import TabItem from '@theme/TabItem';

<TabItem value="common">

import CodeHeader from '@site/src/components/CodeHeader';

<CodeHeader fileName="vector.toml" />

```toml
[transforms.bar]
  type = "regex_parser"
  regex = "^(?P<timestamp>.*) (?P<level>\\w*) (?P<message>.*)$"

[[tests]]
  # REQUIRED - General
  name = "foo test" # example
  
  # REQUIRED - Outputs
  [[tests.outputs]]
    # REQUIRED - General
    extract_from = "bar" # example
    
    # REQUIRED - Conditions
    [[tests.outputs.conditions]]
      # REQUIRED
      type = "check_fields" # example
      
      # OPTIONAL
      name = "message.eq"
    value = "this is the content to match against" # example
      name = "host.exists"
    value = true # example
      name = "method.neq"
    value = "POST" # example
  
  # REQUIRED - Input
  [tests.input]
    # REQUIRED
    type = "raw" # example, enum
    insert_at = "foo" # example
    
    # OPTIONAL
    value = "some message contents" # example, no default, relevant when type = "raw"
```

</TabItem>
<TabItem value="advanced">

<CodeHeader fileName="vector.toml" />

```toml
[transforms.bar]
  type = "regex_parser"
  regex = "^(?P<timestamp>.*) (?P<level>\\w*) (?P<message>.*)$"

[[tests]]
  # REQUIRED - General
  name = "foo test" # example
  
  # REQUIRED - Outputs
  [[tests.outputs]]
    # REQUIRED - General
    extract_from = "bar" # example
    
    # REQUIRED - Conditions
    [[tests.outputs.conditions]]
      # REQUIRED
      type = "check_fields" # example
      
      # OPTIONAL
      name = "message.eq"
    value = "this is the content to match against" # example
      name = "host.exists"
    value = true # example
      name = "method.neq"
    value = "POST" # example
  
  # REQUIRED - Input
  [tests.input]
    # REQUIRED - General
    type = "raw" # example, enum
    insert_at = "foo" # example
    
    # OPTIONAL - General
    value = "some message contents" # example, no default, relevant when type = "raw"
    
    # OPTIONAL - Log fields
    [tests.input.log_fields]
      name = "message"
      value = "some message contents"
    
    # OPTIONAL - Metric
    [tests.input.metric]
      # REQUIRED - General
      type = "counter" # example, enum
      name = "duration_total" # example
      timestamp = "2019-11-01T21:15:47.443232Z" # example
      val = 10.2 # example
      
      # OPTIONAL - General
      direction = "plus" # example, no default, enum
      sample_rate = 1 # example, no default
      
      # OPTIONAL - Tags
      [tests.input.metric.tags]
        name = "host"
        value = "foohost"
```

</TabItem>

</Tabs>

## Options

import Fields from '@site/src/components/Fields';

import Field from '@site/src/components/Field';

<Fields filters={true}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"input"}
  nullable={false}
  path={null}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"table"}
  unit={null}
  >

### input

A table that defines a unit test input event.

<Fields filters={false}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["foo"]}
  name={"insert_at"}
  nullable={false}
  path={"input"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

#### insert_at

The name of a transform, the input event will be delivered to this transform in order to begin the test.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"log_fields"}
  nullable={true}
  path={"input"}
  relevantWhen={{"type":"log"}}
  required={false}
  templateable={false}
  type={"table"}
  unit={null}
  >

#### log_fields

Specifies the log fields when the input type is 'log'.

<Fields filters={false}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[{"name":"message","value":"some message contents"},{"name":"host","value":"myhost"}]}
  name={"`[field-name]`"}
  nullable={false}
  path={"input.log_fields"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"*"}
  unit={null}
  >

##### `[field-name]`

A key/value pair representing a field to be added to the input event.


</Field>


</Fields>

</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"metric"}
  nullable={true}
  path={"input"}
  relevantWhen={{"type":"metric"}}
  required={false}
  templateable={false}
  type={"table"}
  unit={null}
  >

#### metric

Specifies the metric type when the input type is 'metric'.

<Fields filters={false}>


<Field
  common={false}
  defaultValue={null}
  enumValues={{"plus":"Increase the gauge","minus":"Decrease the gauge"}}
  examples={["plus","minus"]}
  name={"direction"}
  nullable={true}
  path={"input.metric"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### direction

The direction to increase or decrease the gauge value.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["duration_total"]}
  name={"name"}
  nullable={false}
  path={"input.metric"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### name

The name of the metric. Defaults to `<field>_total` for `counter` and `<field>` for `gauge`.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[1]}
  name={"sample_rate"}
  nullable={true}
  path={"input.metric"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"float"}
  unit={null}
  >

##### sample_rate

The bucket/distribution the metric is a part of.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"tags"}
  nullable={true}
  path={"input.metric"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"table"}
  unit={null}
  >

##### tags

Key/value pairs representing [metric tags][docs.data-model#tags].

<Fields filters={false}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[{"name":"host","value":"foohost"},{"name":"region","value":"us-east-1"}]}
  name={"`[tag-name]`"}
  nullable={false}
  path={"input.metric.tags"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

###### `[tag-name]`

Key/value pairs representing [metric tags][docs.data-model#tags].


</Field>


</Fields>

</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["2019-11-01T21:15:47.443232Z"]}
  name={"timestamp"}
  nullable={false}
  path={"input.metric"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### timestamp

Time metric was created/ingested.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={{"counter":"A [counter metric type][docs.data-model#counters].","gauge":"A [gauge metric type][docs.data-model#gauges].","histogram":"A [histogram metric type][docs.data-model#histograms].","set":"A [set metric type][docs.data-model#sets]."}}
  examples={["counter"]}
  name={"type"}
  nullable={false}
  path={"input.metric"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### type

The metric type.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[10.2]}
  name={"val"}
  nullable={false}
  path={"input.metric"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"float"}
  unit={null}
  >

##### val

Amount to increment/decrement or gauge.


</Field>


</Fields>

</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={{"raw":"Creates a log event where the message contents are specified in the field 'value'.","log":"Creates a log event where log fields are specified in the table 'log_fields'.","metric":"Creates a metric event, where its type and fields are specified in the table 'metric'."}}
  examples={["raw","log","metric"]}
  name={"type"}
  nullable={false}
  path={"input"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

#### type

The event type.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["some message contents"]}
  name={"value"}
  nullable={true}
  path={"input"}
  relevantWhen={{"type":"raw"}}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

#### value

Specifies the log message field contents when the input type is 'raw'.


</Field>


</Fields>

</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["foo test"]}
  name={"name"}
  nullable={false}
  path={null}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

### name

A unique identifier for this test.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"outputs"}
  nullable={false}
  path={null}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"[table]"}
  unit={null}
  >

### outputs

A table that defines a unit test expected output.

<Fields filters={false}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[]}
  name={"conditions"}
  nullable={false}
  path={"outputs"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"[table]"}
  unit={null}
  >

#### conditions

A table that defines a collection of conditions to check against the output of a transform. A test is considered to have passed when each condition has resolved true for one or more events extracted from the target transform.

<Fields filters={false}>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[{"name":"message.eq","value":"this is the content to match against"}]}
  name={"`<field_name>`.eq"}
  nullable={true}
  path={"outputs.conditions"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### `<field_name>`.eq

Check whether a fields contents exactly matches the value specified.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[{"name":"host.exists","value":true}]}
  name={"`<field_name>`.exists"}
  nullable={true}
  path={"outputs.conditions"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"bool"}
  unit={null}
  >

##### `<field_name>`.exists

Check whether a field exists or does not exist, depending on the provided valuebeing `true` or `false` respectively.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={[{"name":"method.neq","value":"POST"}]}
  name={"`<field_name>`.neq"}
  nullable={true}
  path={"outputs.conditions"}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### `<field_name>`.neq

Check whether a fields contents does not match the value specified.


</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["check_fields"]}
  name={"type"}
  nullable={false}
  path={"outputs.conditions"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

##### type

The type of the condition to execute. Currently only the `check_fields` type is available.


</Field>


</Fields>

</Field>


<Field
  common={true}
  defaultValue={null}
  enumValues={null}
  examples={["bar"]}
  name={"extract_from"}
  nullable={false}
  path={"outputs"}
  relevantWhen={null}
  required={true}
  templateable={false}
  type={"string"}
  unit={null}
  >

#### extract_from

The name of a transform, at the end of the test events extracted from this transform will be checked against a table of conditions.


</Field>


</Fields>

</Field>


</Fields>


[docs.data-model#counters]: /docs/about/data-model#counters
[docs.data-model#gauges]: /docs/about/data-model#gauges
[docs.data-model#histograms]: /docs/about/data-model#histograms
[docs.data-model#sets]: /docs/about/data-model#sets
[docs.data-model#tags]: /docs/about/data-model#tags
