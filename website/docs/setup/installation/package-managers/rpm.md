---
title: Install Vector via RPM
sidebar_label: RPM
description: Install Vector through the RPM package manager
---

<!--
     THIS FILE IS AUTOGENERATED!

     To make changes please edit the template located at:

     website/docs/setup/installation/package-managers/rpm.md.erb
-->

Vector can be installed through the [RPM package manager][urls.rpm] which is
generally used on CentOS.

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

## Install

1.  Download the [Vector `.rpm file`][urls.vector_downloads.0.5.0/vector-x86_64.rpm]

    <Tabs
      className="mini"
      defaultValue="latest"
      values={[
        { label: 'Latest (0.5.0)', value: 'latest'},
        { label: 'Nightly', value: 'nightly'},
      ]}>

    <TabItem value="latest">

    ```bash
    curl -O https://packages.timber.io/vector/0.5.0/vector-x86_64.rpm
    ```

    </TabItem>
    <TabItem value="nightly">

    ```bash
    curl -O https://packages.timber.io/vector/nightly/latest/vector-x86_64.rpm
    ```

    </TabItem>
    </Tabs>

2.  Install the Vector `.rpm` package directly:

    ```bash
    sudo rpm -i vector-x86_64.rpm
    ```

3.  Start Vector:

    ```bash
    sudo systemctl start vector
    ```

    That's it! Proceed to [configure](#configuring) Vector for your use case.

### Previous Versions

Historical Vector versions can be found in the [releases][urls.vector_releases].
Once you've found the version you'd like to install you can re-follow the
[install](#install) steps with the URL to the Vector `.rpm` file.

## Configuring

The Vector configuration file is placed in:

```
etc/vector/vector.toml
```

A full spec is located at `/etc/vector/vector.spec.toml` and examples are
located in `/etc/vector/examples/*`. You can learn more about configuring
Vector in the [Configuration][docs.configuration] section.

## Administering

Vector can be managed through the [Systemd][urls.systemd] service manager:

import Jump from '@site/src/components/Jump';

<Jump to="/docs/administration">Administration</Jump>

## Uninstalling

```bash
sudo rpm -e vector
```

## Updating

Follow the [install](#install) steps again, downloading the latest version of
Vector.


[docs.configuration]: /docs/setup/configuration
[urls.rpm]: https://rpm.org/
[urls.systemd]: https://www.freedesktop.org/wiki/Software/systemd/
[urls.vector_downloads.0.5.0/vector-x86_64.rpm]: https://packages.timber.io/vector/0.5.0/vector-x86_64.rpm
[urls.vector_releases]: https://github.com/timberio/vector/releases
