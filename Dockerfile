# ---- Build stage ----
FROM rust:1.94 AS builder
WORKDIR /app
ENV SQLX_OFFLINE=true

# 1. Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 2. Build actual app
COPY src ./src
COPY .sqlx ./.sqlx
RUN rm -f target/release/deps/mithril* target/release/mithril*
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim AS app
WORKDIR /app
EXPOSE 4000

# RUN apt-get update &&\
#     apt-get install -y libgcc-s1 &&\
#     rm -rf /var/lib/apt/lists/*

# Copy compiled binary
COPY --from=builder /app/target/release/mithril /usr/local/bin/mithril

ENTRYPOINT ["mithril"]
