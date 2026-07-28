all:

prepare-sql:
    DATABASE_URL="mysql://root:password@localhost:3306/combined?ssl-mode=disabled" cargo sqlx prepare

docker-build:
    docker build -t localhost/mithril .

docker-run:
    docker run --rm -p 4000:4000 -e DATABASE_URL_VATUSA="mysql://root:password@mysql:3306/combined?ssl-mode=disabled" -e DATABASE_URL_COBALT="mysql://root:password@mysql:3306/combined?ssl-mode=disabled" --network mithril_default localhost/mithril:latest

test-integration:
    docker build -t localhost/mithril .
    docker compose -f docker-compose.test.yml up -d --wait
    hurl --test --retry 5 --retry-interval 1000 tests/hurl/*.hurl; \
    status=$?; \
    docker compose -f docker-compose.test.yml down -v; \
    exit $status

# Unit + integration coverage combined into one report. Runs the app natively
# (instrumented via `cargo llvm-cov run`) against a dockerized MySQL only, so the
# Hurl suite exercises the same instrumented binary that collects unit test coverage.
# Requires cargo-llvm-cov: https://github.com/taiki-e/cargo-llvm-cov
test-coverage:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo llvm-cov clean --workspace
    docker compose -f docker-compose.test.yml up -d mysql --wait

    APP_PID=""

    # Poll until $1 is gone. Only a graceful exit flushes the LLVM profiling
    # runtime's profraw data, so callers must not short-circuit this.
    wait_for_exit() {
        for _ in $(seq 1 150); do
            kill -0 "$1" 2>/dev/null || return 0
            sleep 0.1
        done
        return 1
    }

    cleanup() {
        status=$?
        if [ -n "$APP_PID" ] && kill -0 "$APP_PID" 2>/dev/null; then
            kill -TERM "$APP_PID" 2>/dev/null || true
            wait_for_exit "$APP_PID" || kill -KILL "$APP_PID" 2>/dev/null || true
        fi
        docker compose -f docker-compose.test.yml down -v
        exit $status
    }
    trap cleanup EXIT

    cargo llvm-cov --no-report

    export DATABASE_URL_VATUSA="mysql://root:password@localhost:3306/combined?ssl-mode=disabled"
    export DATABASE_URL_COBALT="mysql://root:password@localhost:3306/combined?ssl-mode=disabled"

    # Compile up front so the readiness wait below times a starting process
    # rather than a build.
    cargo llvm-cov run --no-report -- --version >/dev/null

    cargo llvm-cov run --no-report -- --host 0.0.0.0 --port 4000 &
    CARGO_PID=$!

    # `cargo run` spawns the binary as a child, so $! is cargo's pid, not the
    # app's. Anchor the pattern so it matches only the app and never the
    # surrounding shell's own command line.
    for _ in $(seq 1 300); do
        APP_PID=$(pgrep -f "^target/llvm-cov-target/debug/mithril --host" || true)
        [ -n "$APP_PID" ] && break
        sleep 0.1
    done
    [ -n "$APP_PID" ] || { echo "app process never started" >&2; exit 1; }

    for _ in $(seq 1 300); do
        curl -sf -o /dev/null http://localhost:4000/health && break
        sleep 0.1
    done
    curl -sf -o /dev/null http://localhost:4000/health \
        || { echo "app never became ready" >&2; exit 1; }

    hurl --test --retry 5 --retry-interval 1000 tests/hurl/*.hurl

    kill -TERM "$APP_PID"
    wait_for_exit "$APP_PID" \
        || { echo "app ignored SIGTERM; coverage data was never flushed" >&2; exit 1; }
    wait "$CARGO_PID" 2>/dev/null || true
    APP_PID=""

    cargo llvm-cov report --html
    cargo llvm-cov report --lcov --output-path target/llvm-cov/lcov.info
    cargo llvm-cov report --summary-only
