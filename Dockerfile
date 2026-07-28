# ---- Chef stage (shared base with cargo-chef installed) ----
FROM rust:1.97.1@sha256:1bcff4befb740599103a2c7cb51058e14479b2e35e3a34a3f0dc4ede09927488 AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

# ---- Planner: compute the dependency recipe from Cargo.toml/Cargo.lock ----
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# ---- Build stage ----
FROM chef AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true

# 1. Build just the dependencies. This layer is cached by Docker/GHA layer
# caching keyed on recipe.json, so it's only invalidated when Cargo.toml or
# Cargo.lock change (unlike `RUN --mount=type=cache`, which GHA's cache
# backend does not persist across ephemeral runners).
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 2. Build actual app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx
RUN cargo build --release && cp target/release/mithril /app/mithril

# ---- Runtime stage ----
FROM debian:trixie-slim@sha256:020c0d20b9880058cbe785a9db107156c3c75c2ac944a6aa7ab59f2add76a7bd AS app
WORKDIR /app
EXPOSE 4000

# reqwest (rustls) loads root certs from the system trust store at runtime;
# debian-slim doesn't ship it by default.
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/mithril /usr/local/bin/mithril

ENTRYPOINT ["mithril"]
