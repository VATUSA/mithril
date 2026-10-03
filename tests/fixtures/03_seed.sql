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

-- A non-ZHQ, facility-scoped key (ZHU), so tests can verify that a
-- facility-bound key only sees its own facility's unapproved events
-- (plus anything approved), not every facility's events like a ZHQ key does.
INSERT INTO `v3_api_key` (`code`, `testing`, `facility`, `notes`, `created_at`)
VALUES ('test-facility-key', 0, 'ZHU', 'seeded for facility-scoped visibility tests', UNIX_TIMESTAMP());

-- One controller visiting another facility, plus a facility-scoped and a
-- division-wide role grant, for the /division/controllers dump. The ZZN and
-- ZAE rows must be left out of that dump.
INSERT INTO `user` (`cid`, `display_name`, `controller_rating`, `facility`)
VALUES (1000001, 'Test Controller', 5, 'ZHU'),
       (1000002, 'Test Non-Member', 1, 'ZZN'),
       (1000003, 'Test Academy', 1, 'ZAE');
INSERT INTO `user_visit` (`cid`, `facility`) VALUES (1000001, 'ZJX');
INSERT INTO `acl_user_role` (`cid`, `facility`, `role`, `grantor_cid`, `granted_at`)
VALUES (1000001, 'ZHU', 'mentor', 0, 0),
       (1000001, '*', 'ace_team', 0, 0);
