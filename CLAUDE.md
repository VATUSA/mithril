# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.
For how this repo relates to the other VATUSA projects, see the workspace `CLAUDE.md` one directory up.

## Project Overview

Mithril is a new VATUSA HTTP API written in Rust. It provides programmatic access for ARTCCs (air traffic control centers) and facilities to interact with the VATUSA system. The API uses X-API-Key header authentication and supports both public endpoints and facility-specific operations.

The project is designed to replace/extend the existing legacy API by providing a modern Rust-based backend built on Axum (async web framework).

## Role in the larger VATUSA setup

Mithril is the intended **"v3" API** — the long-term successor to the legacy Laravel API
(`api`) and a sibling to the Go `cobalt` backend. Beyond being just another backend, it
has a specific job during the platform migration:

- **API-contract continuity during migration.** Mithril acts as a stable facade that holds
  the external API contract steady while the website (`site`) and the underlying
  data model are migrated underneath it. Consumers code against mithril's v3 contract; the
  storage and ownership of data can move without breaking them.
- **Straddling old and new data models.** This is why mithril connects to **two databases**
  at once (see Code Architecture below): `vatusa_db` (the legacy "vatusa-old" schema) and
  `cobalt_db` (the new backend schema). It can read legacy data and write/serve new-model
  data through one consistent surface, letting tables migrate from old to new incrementally
  rather than in a big-bang cutover.
- **Where it sits.** Legacy pair (`site` + `api`) → newer backends
  (`cobalt`, mithril) → modern frontend (`webapps`). As migration proceeds, responsibility
  shifts from the legacy Laravel stack toward mithril/cobalt. When changing mithril's API
  contract, treat it as a published interface and check downstream consumers.

See the workspace `CLAUDE.md` for the full project map and the migration strategy notes.

## Build & Development Commands

### Using just (task runner)

```sh
just prepare-sql          # Prepare SQLx offline query metadata
just docker-build         # Build Docker image
just docker-run           # Run Docker container with MySQL
```

### Using cargo directly

```sh
cargo build               # Debug build
cargo build --release     # Release build
cargo test                # Run all tests
cargo +nightly clippy     # Run linter (requires nightly Rust)
cargo run                 # Run the application (defaults to 0.0.0.0:4000)
cargo run -- --host 127.0.0.1 --port 8080  # Run with custom host/port
```

### Pre-push hooks (automatic)

The project uses rusty-hook with pre-push checks:
- `cargo b` (build)
- `cargo t` (test)
- `cargo +nightly clippy` (linting)

These run automatically before git push. To bypass (not recommended): use `git push --no-verify`.

### Requirements
- Rust (recent stable + nightly for clippy linting)
- MySQL 9 (for database)
- Docker & Docker Compose (for containerized development)

## Code Architecture

### High-Level Structure

**Framework**: Axum (async web framework) with OpenAPI documentation via utoipa

**Two separate databases**:
- `vatusa_db`: Legacy "vatusa-old" database (read-heavy, contains controllers, facilities, ratings, etc.)
- `cobalt_db`: New VATUSA backend database (write-heavy, contains news posts, events)

**Request flow**:
1. HTTP request arrives at Axum router
2. Authentication middleware (`middleware.rs`) validates X-API-Key header (if present)
3. Auth context inserted into request extensions (Auth::Anonymous or Auth::Key)
4. Route handler executes and queries database via sqlx
5. Response serialized to JSON with OpenAPI schema

### Module Organization

- **`main.rs`**: App setup, router configuration, graceful shutdown, OpenAPI documentation
- **`middleware.rs`**: Authentication middleware, extractors for optional/required auth
- **`db.rs`**: Database connection pools and all data model structs (using sqlx::FromRow)
- **`queries.rs`**: CRUD operations organized by table (get, create, update, delete patterns)
- **`shared.rs`**: AppState, error handling (AppError enum), Auth enum, facility determination logic
- **`routes/`**: Endpoint implementations, organized by domain (news, events, facility, etc.)

### Route Pattern

Each route module follows a standard pattern:

```rust
// 1. Define router function that composes OpenAPI routes
pub fn router(state: Arc<AppState>) -> OpenApiRouter { ... }

// 2. Define handlers with #[utoipa::path] macros for OpenAPI docs
async fn handler(...) -> Result<Json<T>, AppError> { ... }

// 3. Route handlers use:
//    - State(state): Arc<AppState> for database access
//    - RequireAuth: for protected endpoints (returns 401 if no API key)
//    - AuthExtractor: for optional auth that may control response data
```

### Authentication & Authorization

**Three auth patterns**:
1. **Anonymous**: No X-API-Key header → public endpoints only
2. **OptionalAuth** (AuthExtractor): May include key; no error if missing
3. **RequireAuth**: Must include valid X-API-Key; returns 401 if missing/invalid

**Key attributes** (from v3_api_key table):
- `testing`: If true, operations log but don't execute (for sandbox testing)
- `facility`: Optional; restricts key to specific facility (ZHQ keys can override)

**Facility determination** (`determine_facility` helper):
- ZHQ keys can specify facility in request or default to "ZHQ"
- Non-ZHQ keys locked to their assigned facility
- Returns error if facility can't be determined

### Data Access Patterns

**sqlx usage**:
- Uses `sqlx::query_as!` macro for compile-time query validation
- Requires `.sqlx/` directory with offline metadata (see `justfile prepare-sql`)
- All queries use parameterized inputs (? placeholders) to prevent SQL injection
- Connection pools (MySqlPool) are shared via AppState

**Query organization** (`queries.rs`):
- Functions grouped by table name (comments demarcate sections)
- CRUD operations follow naming: `get_*`, `create_*`, `update_*`, `delete_*`
- Request bodies are separate Deserialize structs (CreateNewsPost, UpdateEvent, etc.)
- HasFacility trait for payloads that include optional facility field

### Error Handling

**AppError enum** maps to HTTP status codes:
- 401 Unauthorized: ApiKeyRequired
- 403 Forbidden: InsufficientPermissions
- 400 Bad Request: BadRequest, JsonProcessingError
- 404 Not Found: NotFound, RouteNotFound
- 500 Internal Server Error: Database, EnvVarError, Internal

**Error responses** are JSON with structure:
```json
{
  "error": {
    "code": "api_key_required",
    "message": "API key required"
  }
}
```

Server errors log via tracing (WARN for client errors, ERROR for 5xx).

## Database Setup

### Connection strings

Set via environment variables:
- `DATABASE_URL_VATUSA`: e.g., `mysql://root:password@localhost:3306/combined?ssl-mode=disabled`
- `DATABASE_URL_COBALT`: e.g., `mysql://root:password@localhost:3306/combined?ssl-mode=disabled`

### Docker Compose

`docker-compose.yml` provides MySQL 9 service:
- Host: localhost:3306
- Root user: root
- Password: password
- Database: combined

Start with: `docker-compose up -d`

### Migrations

**Important**: This codebase does NOT use traditional migrations. The databases are pre-existing and populated. The .sqlx/ directory contains offline metadata for compile-time query validation. To add new queries or modify existing ones:

1. Update code with new sqlx::query! macro
2. Run `just prepare-sql` (requires live database connection)
3. Commit the updated `.sqlx/` directory

### Data Models

All models in `db.rs` derive:
- `FromRow`: For sqlx query results
- `Serialize`: For JSON responses
- `ToSchema`: For OpenAPI documentation

Tables include: controllers, facilities, news_post, event, training_records, solo_certs, ratings, roles, and many others related to VATUSA operations.

## Testing Flag Behavior

API keys can have a `testing` flag. When true:
- Operations log but do NOT execute
- Useful for integration testing without side effects
- Check `auth.testing` in handlers before executing mutations

Example pattern (from news.rs, events.rs):
```rust
if !auth.testing {
    // Perform actual database write
    queries::create_news_post(...).await?;
    tracing::info!("key {} created post", auth.key_id);
} else {
    tracing::debug!("testing key {} on endpoint", auth.key_id);
}

```

## Integration Testing

Integration tests are HTTP-level (not Rust test code), using [Hurl](https://hurl.dev/)
against a real, ephemeral stack:

- `docker-compose.test.yml` runs MySQL + the built mithril image. MySQL is bootstrapped
  from schema dumps in `tests/fixtures/` (`01_cobalt_schema.sql`, `02_vatusa_old_schema.sql`,
  numbered so they load in order via `docker-entrypoint-initdb.d`) plus `03_seed.sql`,
  which inserts two `v3_api_key` rows: a ZHQ-facility, non-testing key (`test-zhq-key`)
  used by the CRUD scenarios, and a ZHQ-facility key with `testing = 1` (`test-testing-key`)
  used to verify testing-flagged keys never persist writes. ZHQ keys skip the
  `cid_in_facility` roster check, so no controller/facility fixture data is needed to
  exercise writes.
- `tests/hurl/news.hurl` and `tests/hurl/events.hurl` contain create → read → update →
  read → delete → read scenarios for their data type. Hurl's `[Captures]` chain IDs
  between steps. `tests/hurl/facility.hurl` is a read-only smoke test (the facility
  routes have no write endpoints implemented yet). `tests/hurl/testing_key.hurl` posts
  to news and events with the testing-flagged key and asserts nothing was persisted.
- Hurl's jsonpath filter (`$[?(@.field=='x')]`) unwraps a single match to a scalar and
  errors on chained predicates like `count` or `nth` in that case — avoid combining a
  filter expression with those. When asserting "no matching row," assert `jsonpath "$"
  count == N` against the full list instead of filtering for zero matches, since a
  filter with zero matches also isn't a plain empty list from Hurl's engine.
- Run everything with `just test-integration`: builds the image, brings the compose
  stack up, runs Hurl, then tears the stack down with `down -v` so the MySQL volume
  is discarded and the next run starts from a clean database.
- The two schema dumps are regenerated occasionally (`mysqldump --no-data`) when the
  `cobalt`/`vatusa_old` schemas change — they are not expected to change often, so no
  automation refreshes them.

## Code Coverage

`just test-coverage` produces one merged coverage report for unit tests *and* the Hurl
integration suite, using [cargo-llvm-cov](https://github.com/taiki-e/cargo-llvm-cov):

- Unit tests run via `cargo llvm-cov --no-report` (writes profraw data, no report yet).
- The app is then run natively — not the Docker image — via `cargo llvm-cov run --no-report
  -- --host 0.0.0.0 --port 4000`, instrumented the same way, against only the `mysql`
  service from `docker-compose.test.yml` (started standalone, not the full stack). This
  keeps the Hurl requests hitting the same instrumented binary that's collecting unit
  test coverage, so both merge into one profile.
- Graceful shutdown matters here: the recipe sends the app process `SIGTERM` after Hurl
  finishes, and only a clean exit (not a kill -9) flushes the LLVM profiling runtime's
  profraw data — this already works because the app's existing SIGTERM handler
  (`shutdown_signal` in `main.rs`) causes `main` to return normally.
- `cargo llvm-cov run` (not a manual `cargo llvm-cov show-env` + `cargo build`) is
  required to get the instrumented binary — sourcing `show-env` and building manually
  left the profiled and non-profiled binaries in different target dirs and cargo didn't
  reliably invalidate/rebuild the cached one, silently producing an uninstrumented binary
  with no profraw output at all.
- `cargo llvm-cov report --html` merges every profraw file found under
  `target/llvm-cov-target/` into `target/llvm-cov/html/index.html`; the recipe also
  emits `--lcov --output-path target/llvm-cov/lcov.info` and `--summary-only` in the
  same run.
- CI (`.github/workflows/validate.yml`, `coverage` job) installs `just`/`cargo-llvm-cov`
  via `taiki-e/install-action`, installs `hurl` from its `.deb` GitHub release (not
  listed in `taiki-e/install-action`'s tool manifest), runs `just test-coverage`, and
  uploads `target/llvm-cov/lcov.info` to Codecov via `codecov/codecov-action`. The
  README's coverage badge points at `codecov.io/gh/vatusa/mithril`. A `CODECOV_TOKEN`
  repo secret should be set (required for reliable uploads even on public repos since
  Codecov tightened tokenless uploads).

## Code Quality Standards

- **Linting**: `cargo +nightly clippy` enforces all Clippy lints (no warnings)
- **Unsafe code**: Denied via `#![deny(unsafe_code)]` in main.rs
- **Edition**: 2024 (latest Rust edition at time of creation)
- **Dependencies**: Minimal and well-maintained (axum, sqlx, tokio, utoipa, etc.)

## Notable Implementation Details

- **Graceful shutdown**: Listens for SIGTERM (unix) and SIGINT (Ctrl+C)
- **Request tracing**: tower-http TraceLayer logs all requests
- **Request timeout**: 60-second timeout for all requests (TimeoutLayer)
- **Rate limiting**: Uses tower_governor (configured but integration pending)
- **Session support**: tower-sessions available (not currently used)
- **Logging**: tracing-subscriber with DEBUG level by default
- **JSON errors**: Custom error responses prevent internal details leakage
