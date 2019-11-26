---
title: Unit Testing Guide
sidebar_label: Unit Testing
description: Learn how to write and execute unit tests for your configs
---

It's possible to define unit tests within a Vector configuration file that cover
a network of transforms within the topology. The intention of these tests is to
improve the maintainability of configs containing larger and more complex
combinations of transforms.

This guide will cover writing and executing a unit test for the following config:

import CodeHeader from '@site/src/components/CodeHeader';

<CodeHeader fileName="example.toml" />

```toml
[sources.over_tcp]
  type = "tcp"
  address = "0.0.0.0:9000"

[transforms.foo]
  type = "grok_parser"
  inputs = ["over_tcp"]
  pattern = "%{TIMESTAMP_ISO8601:timestamp} %{LOGLEVEL:level} %{GREEDYDATA:message}"

[transforms.bar]
  type = "add_fields"
  inputs = ["foo"]
  [transforms.bar.fields]
    new_field = "this is a static value"

[transforms.baz]
  type = "remove_fields"
  inputs = ["bar"]
  fields = ["level"]

[sinks.over_http]
  type = "http"
  inputs = ["baz"]
  uri = "http://localhost:4195/post"
  encoding = "text"
```

In this config we:

- Parse a log line into the fields `timestamp`, `level` and `message` with the
  transform `foo`.
- Add a static string field `new_field` using the transform `bar`.
- Remove the field `level` with the transform `baz`.

TODO

Executing tests within a config file can be done with the `test` subcommand:

```bash
vector test ./example.toml
```

[docs.testing]: /docs/reference/testing
