---
title: Install Vector On Raspberry Pi
sidebar_label: Raspberry Pi
description: Install Vector On Raspberry Pi
---

Vector can be installed on Raspberry Pi through the following methods:

import Jump from '@site/src/components/Jump';
import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs
  block={true}
  defaultValue="arm64"
  urlKey="os"
  values={[
    { label: 'ARM64', value: 'arm64', },
    { label: 'ARMv7', value: 'armhf', },
    ]}>

<TabItem value="arm64">

<Jump to="/docs/setup/installation/package-managers/dpkg?arch=arm64">
  <i className="feather icon-package"></i> DPKG (ARM64)<span class="badge badge--primary">recommended</span>
</Jump>

Alternatively, if you prefer manual installation:

<Jump to="/docs/setup/installation/manual/from-archives?os=linux_arm64" size="sm">
  <i className="feather icon-terminal"></i> Install From an Archive (ARM64)
</Jump>
<Jump to="/docs/setup/installation/manual/from-source" size="sm">
  <i className="feather icon-terminal"></i> Install From Source
</Jump>

</TabItem>
<TabItem value="armhf">

<Jump to="/docs/setup/installation/package-managers/dpkg?arch=armhf">
  <i className="feather icon-package"></i> DPKG (ARMv7) <span class="badge badge--primary">recommended</span>
</Jump>

Alternatively, if you prefer manual installation:

<Jump to="/docs/setup/installation/manual/from-archives?os=linux_armv7" size="sm">
  <i className="feather icon-terminal"></i> Install From an Archive (ARMv7)
</Jump>
<Jump to="/docs/setup/installation/manual/from-source" size="sm">
  <i className="feather icon-terminal"></i> Install From Source
</Jump>

</TabItem>
</Tabs>



