---
title: Global Options
description: Global configuration options
---

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     website/docs/reference/global-options.md.erb
-->

In addition to [sources][docs.sources], [transforms][docs.transforms], and
[sinks][docs.sinks], Vector accepts global options that serve to configure
Vector as a whole and set defaults for component options.

## Configuration

import CodeHeader from '@site/src/components/CodeHeader';

<CodeHeader fileName="vector.toml" />

```toml
data_dir = "/var/lib/vector" # example, no default
  dns_servers = ["0.0.0.0:53"] # example, no default
```

## Options

import Fields from '@site/src/components/Fields';

import Field from '@site/src/components/Field';

<Fields filters={true}>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={["/var/lib/vector"]}
  name={"data_dir"}
  nullable={true}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"string"}
  unit={null}
  >

### data_dir

The directory used for persisting Vector state, such as on-disk buffers, file checkpoints, and more. Please make sure the Vector project has write permissions to this dir. See [Data Directory](#data-directory) for more info.


</Field>


<Field
  common={false}
  defaultValue={null}
  enumValues={null}
  examples={[["0.0.0.0:53"]]}
  name={"dns_servers"}
  nullable={true}
  path={null}
  relevantWhen={null}
  required={false}
  templateable={false}
  type={"[string]"}
  unit={null}
  >

### dns_servers

The list of DNS servers Vector will use to resolve DNS requests. When set Vector will ignore the system configuration and use only the list of DNS servers provided. If this option is not set then Vector will attempt to use the system configuration. 


</Field>


</Fields>

## How It Works

### Data Directory

Vector requires a[`data_dir`](#data_dir) value for on-disk operations. Currently, the only
operation using this directory are Vector's on-disk buffers. Buffers, by
default, are memory-based, but if you switch them to disk-based you'll need to
specify a[`data_dir`](#data_dir).


[docs.sinks]: /docs/reference/sinks
[docs.sources]: /docs/reference/sources
[docs.transforms]: /docs/reference/transforms
