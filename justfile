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

    cleanup() {
        status=$?
        [ -n "${APP_PID:-}" ] && kill -TERM "$APP_PID" 2>/dev/null && wait "$APP_PID" 2>/dev/null || true
        docker compose -f docker-compose.test.yml down -v
        exit $status
    }
    trap cleanup EXIT

    cargo llvm-cov --no-report

    export DATABASE_URL_VATUSA="mysql://root:password@localhost:3306/combined?ssl-mode=disabled"
    export DATABASE_URL_COBALT="mysql://root:password@localhost:3306/combined?ssl-mode=disabled"
    cargo llvm-cov run --no-report -- --host 0.0.0.0 --port 4000 &
    for i in $(seq 1 30); do curl -s -o /dev/null http://localhost:4000/facility && break; sleep 0.3; done
    APP_PID=$(pgrep -f "target/llvm-cov-target/debug/mithril")

    hurl --test --retry 5 --retry-interval 1000 tests/hurl/*.hurl

    kill -TERM "$APP_PID"
    wait "$APP_PID" 2>/dev/null || true
    APP_PID=""

    cargo llvm-cov report --html
    cargo llvm-cov report --lcov --output-path target/llvm-cov/lcov.info
    cargo llvm-cov report --summary-only
