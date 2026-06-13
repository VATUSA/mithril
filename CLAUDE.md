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
  the external API contract steady while the website (`current_site`) and the underlying
  data model are migrated underneath it. Consumers code against mithril's v3 contract; the
  storage and ownership of data can move without breaking them.
- **Straddling old and new data models.** This is why mithril connects to **two databases**
  at once (see Code Architecture below): `vatusa_db` (the legacy "vatusa-old" schema) and
  `cobalt_db` (the new backend schema). It can read legacy data and write/serve new-model
  data through one consistent surface, letting tables migrate from old to new incrementally
  rather than in a big-bang cutover.
- **Where it sits.** Legacy pair (`current_site` + `current_api`) → newer backends
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
    tracing::info!("Key {} created post", auth.key_id);
} else {
    tracing::debug!("Testing key {} on endpoint", auth.key_id);
}

```
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

