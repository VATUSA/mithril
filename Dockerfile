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
RUN cargo build --release

# ---- Runtime stage ----
FROM alpine:3.23
WORKDIR /app
EXPOSE 4000

# Copy compiled binary
COPY --from=builder /app/target/release/mithril /usr/local/bin/mithril

CMD ["mithril"]
