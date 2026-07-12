# mithril

![lang](https://img.shields.io/badge/lang-rust-orange)
![licensing](https://img.shields.io/badge/license-MIT-green)
![status](https://img.shields.io/badge/project_status-in_dev-yellow)

New VATUSA API for facilities and guests.

All content herein is solely for use on the [VATSIM network](https://vatsim.net/).

## Project goals

TBD

## Building

### Requirements

- Git
- A recent version of [Rust](https://www.rust-lang.org/tools/install)

### Steps

```sh
git clone https://github.com/vatusa/mithril
cd mithril
cargo build
```

This app follows all [Clippy](https://doc.rust-lang.org/clippy/) lints on _Nightly Rust_. You can use either both a stable and nightly toolchain, or just a nightly (probably; I use the dual setup). If using both, execute clippy with `cargo +nightly clippy`. You do not need this for _running_ the app, just developing on it.

## Running

TBD

### Environment variables

- `MITHRIL_ROSTER_POLL`: when set to `TRUE`, enable the roster change poll task

## Deploying

TBD

## License

See [LICENSE.md](./LICENSE.md).

## Contributing

Contributions are currently closed to anyone not on the VATUSA Web Team.
