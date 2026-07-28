# Verifies the `testing` flag on an API key: the endpoint still responds as
# if the write succeeded, but no row is actually persisted. Covers news,
# events, and webhooks, the write-capable data types with CRUD tests in this
# suite.

POST http://localhost:4000/news
X-API-Key: test-testing-key
Content-Type: application/json
{
    "title": "Testing key news post",
    "body": "Should never be persisted",
    "author_cid": 1000002
}
HTTP 201

GET http://localhost:4000/news
X-API-Key: test-testing-key
HTTP 200
[Asserts]
jsonpath "$" count == 0


POST http://localhost:4000/events
X-API-Key: test-testing-key
Content-Type: application/json
{
    "title": "Testing key event",
    "body": "Should never be persisted",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 1893456000,
    "end_time": 1893459600,
    "created_by": 1000002
}
HTTP 201

GET http://localhost:4000/events
X-API-Key: test-testing-key
HTTP 200
[Asserts]
jsonpath "$" count == 0


POST http://localhost:4000/webhooks
X-API-Key: test-testing-key
Content-Type: application/json
{
    "url": "https://example.com/testing-webhook"
}
HTTP 201

GET http://localhost:4000/webhooks
X-API-Key: test-testing-key
HTTP 200
[Asserts]
jsonpath "$" count == 0
