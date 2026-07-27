-- Minimal seed data for integration tests, loaded after the schema dumps.
-- A single ZHQ-facility, non-testing API key so CRUD tests exercise real
-- writes without needing per-facility roster fixtures (ZHQ keys bypass the
-- `cid_in_facility` check in the route handlers).
INSERT INTO `v3_api_key` (`code`, `testing`, `facility`, `notes`, `created_at`)
VALUES ('test-zhq-key', 0, 'ZHQ', 'seeded for integration tests', UNIX_TIMESTAMP());

-- A ZHQ-facility key with `testing` set, so tests can verify that keys
-- flagged for testing log intent but never actually write to the DB.
INSERT INTO `v3_api_key` (`code`, `testing`, `facility`, `notes`, `created_at`)
VALUES ('test-testing-key', 1, 'ZHQ', 'seeded for testing-flag integration tests', UNIX_TIMESTAMP());
