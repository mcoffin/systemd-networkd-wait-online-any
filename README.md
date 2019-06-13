# systemd-networkd-wait-online-any

**NOTICE**: As of [`v242`](https://github.com/systemd/systemd/releases/tag/v242) of systemd, `systemd-networkd-wait-online` accepts the `--any` option on the command line. This project is now **deprecated**.

Wrapper for `systemd-networkd-wait-online` that will wait until **one** interface is ready instead of waiting for all interfaces to be ready.

# Deprecation notice

As of systemd/systemd#12160, this project is now deprecated in favor of the `--any` flag passable to `systemd-networkd-wait-online`.
