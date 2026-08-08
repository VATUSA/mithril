# mithril

![lang](https://img.shields.io/badge/lang-rust-orange)
![licensing](https://img.shields.io/badge/license-MIT-green)
[![Validate](https://github.com/VATUSA/mithril/actions/workflows/validate.yml/badge.svg)](https://github.com/VATUSA/mithril/actions/workflows/validate.yml)
[![codecov](https://codecov.io/gh/vatusa/mithril/branch/master/graph/badge.svg?token=5K7BESBCJZ)](https://codecov.io/gh/vatusa/mithril)

New VATUSA API for facilities and guests.

All content herein is solely for use on the [VATSIM network](https://vatsim.net/).

## Project goals

1. Completely replace the existing VATUSA API for subdivisions and other clients
2. Be accurate, consistent, and correct
3. Be fast

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

### Environment Variables

- `DISABLE_ROSTER_POLLER`: Set to `true` or `1` to disable the roster change poller background task. By default, the poller runs and periodically syncs roster changes. Set this to disable polling in development or testing scenarios.
- `DB_MAX_CONNECTIONS`: Maximum number of connections in each database pool (`vatusa_db` and `cobalt_db`). Defaults to `5` if unset or invalid.

## Enforced quality

Rust allows us to enforce quality much earlier on in the development cycle than other languages. For one: the program has to compile, satisfying the notoriously picky Rust compiler. Second, the [clippy](<https://github.com/rust-lang/rust-clippy>) tool enforces many additional lints to not only correct potentially bad code but to also enforce good habits. No [unsafe](<https://doc.rust-lang.org/std/keyword.unsafe.html>) code is allowed in this project. Next, the [library used](<https://crates.io/crates/sqlx>) for the database connection is ran in a mode that verifies queries at **compile time** against an actual running database (or the cached verified files of that process). Finally, integration tests with code coverage exercise the actual (local) E2E functionality with a really database via Docker.

All of these processes will be enforced at all times with the exceptions of code that is not yet implemented or specific pedantic clippy lints.

### Integration testing

Integration tests use [Hurl](https://hurl.dev/) to exercise the running HTTP API against
an ephemeral, schema-only MySQL instance (no JVM, no test code to maintain).

```sh
just test-integration
```

This builds the app image, brings up `docker-compose.test.yml` (MySQL seeded from
`tests/fixtures/*.sql`, plus the mithril app container), runs every `*.hurl` file in
`tests/hurl/`, then tears the stack down (`down -v`) so the next run starts from a clean
database. See `tests/fixtures/` for the schema dumps and seed data, and `tests/hurl/`
for the test scenarios.

### Code coverage

Combined unit + integration test coverage, via [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov):

```sh
just test-coverage
```

This runs `cargo test`'s unit tests, then runs the app natively (instrumented by
`cargo llvm-cov run`) against a dockerized MySQL-only instance and exercises it with the
same `tests/hurl/*.hurl` suite used by `just test-integration`, so both test styles feed
one merged report. Output: a terminal summary, an HTML report at
`target/llvm-cov/html/index.html`, and an lcov file at `target/llvm-cov/lcov.info`.

CI runs the same recipe on every push/PR (`.github/workflows/validate.yml`, `coverage`
job) and uploads the lcov file to [Codecov](https://codecov.io/gh/vatusa/mithril), which
is what the badge above reflects.

## Deploying

TBD

## License

See [LICENSE.md](./LICENSE.md).

## Contributing

Contributions are currently closed to anyone not on the VATUSA Web Team.
