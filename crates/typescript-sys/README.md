# TypeScript system Rust bindings

## Development

There are three supported backends: QuickJS (the default) via [`rquickjs`](https://docs.rs/rquickjs), SpiderMonkey via [`mozjs`](https://docs.rs/mozjs), and JavaScriptCore via [`javascriptcore-rs`](https://docs.rs/javascriptcore-rs). All of these backends are optional features. You must select at least one backend to enable.

- x86_64-unknown-linux-gnu
- aarch64-unknown-linux-gnu
- x86_64-pc-windows-msvc
- x86_64-apple-darwin
- aarch64-apple-darwin