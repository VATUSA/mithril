# ---- Chef stage (shared base with cargo-chef installed) ----
# --platform=$BUILDPLATFORM keeps this stage on the runner's native arch and
# lets cargo cross-compile to $TARGETARCH instead of emulating the whole
# toolchain under QEMU (a release build under emulation is 30-60+ min).
FROM --platform=$BUILDPLATFORM rust:1.97.1@sha256:1bcff4befb740599103a2c7cb51058e14479b2e35e3a34a3f0dc4ede09927488 AS chef
WORKDIR /app
RUN cargo install cargo-chef --locked

# Map Docker's TARGETARCH to a Rust target triple once and stash it in a file
# — ENV can't be set dynamically from a RUN's output, so every later stage
# built FROM chef reads /rust_target instead of recomputing the mapping.
ARG TARGETARCH
RUN case "$TARGETARCH" in \
      amd64) echo x86_64-unknown-linux-gnu > /rust_target ;; \
      arm64) echo aarch64-unknown-linux-gnu > /rust_target ;; \
      *) echo "unsupported TARGETARCH: $TARGETARCH" >&2; exit 1 ;; \
    esac
RUN rustup target add "$(cat /rust_target)"
# gcc-aarch64-linux-gnu + libc6-dev-arm64-cross are only needed cross-building
# to arm64 from an amd64 builder; the amd64 target links with the image's
# native toolchain. reqwest is on rustls, but rustls's default crypto
# provider (aws-lc-rs) still has a small C component (aws-lc-sys) that needs
# a real cross C toolchain — gcc-aarch64-linux-gnu alone is not sufficient,
# it needs the target's libc headers (sys/types.h etc.) too.
RUN if [ "$TARGETARCH" = "arm64" ]; then \
      apt-get update && apt-get install -y --no-install-recommends \
        gcc-aarch64-linux-gnu libc6-dev-arm64-cross \
      && rm -rf /var/lib/apt/lists/* ; \
    fi
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

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
RUN cargo chef cook --release --target "$(cat /rust_target)" --recipe-path recipe.json

# 2. Build actual app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx
RUN cargo build --release --target "$(cat /rust_target)" \
    && cp target/"$(cat /rust_target)"/release/mithril /app/mithril

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
