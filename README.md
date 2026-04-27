# my-malbox-plugin

A [malbox](https://github.com/DualHorizon/malbox) plugin template for Rust.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)

## Getting started

1. Click **Use this template** on GitHub to create your own repository.
2. Rename your plugin in `Cargo.toml` (the `name` field) and `plugin.toml` (the `[plugin]` section).
3. Build:

```bash
cargo build
```

## Project structure

| File | Purpose |
|------|---------|
| `Cargo.toml` | Rust package configuration and dependencies |
| `src/main.rs` | Plugin implementation |
| `plugin.toml` | Plugin metadata, runtime config, and result declarations |
| `build.rs` | Watches `plugin.toml` for changes |

## Switching to a host plugin

By default this template creates a **guest** plugin (runs inside the analysis VM). To create a **host** plugin (runs on the host machine):

1. In `Cargo.toml`, change `features = ["guest"]` to `features = ["host"]`
2. In `src/main.rs`, change `#[malbox::guest_plugin]` to `#[malbox::host_plugin]`
3. In `plugin.toml`, change `type = "guest"` to `type = "host"`

## Further reading

- [Creating a Rust plugin](https://docs.malbox.app/reference/plugin-sdk/rust)
- [Plugin SDK reference](https://crates.malbox.app/malbox_plugin_sdk/index.html)
- [Plugin configuration reference](https://docs.malbox.app/reference/configuration/plugins)
