# ---- Build stage ----
FROM rust:1.97.1@sha256:1bcff4befb740599103a2c7cb51058e14479b2e35e3a34a3f0dc4ede09927488 AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true

# 1. Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release
RUN rm -rf src

# 2. Build actual app
COPY src ./src
COPY .sqlx ./.sqlx
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    rm -f target/release/deps/mithril* target/release/mithril* && \
    cargo build --release && \
    cp target/release/mithril /app/mithril

# ---- Runtime stage ----
FROM debian:trixie-slim@sha256:020c0d20b9880058cbe785a9db107156c3c75c2ac944a6aa7ab59f2add76a7bd AS app
WORKDIR /app
EXPOSE 4000

# Copy compiled binary
COPY --from=builder /app/mithril /usr/local/bin/mithril

ENTRYPOINT ["mithril"]
