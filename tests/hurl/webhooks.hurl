# Lifecycle for /webhooks: create -> read -> delete -> read, plus the https-only
# URL validation. There's no update endpoint, so no PATCH step here.
# Run against a freshly-seeded stack; see docker-compose.test.yml.

POST http://localhost:4000/webhooks
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "url": "http://example.com/webhook"
}
HTTP 400


POST http://localhost:4000/webhooks
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "url": "https://example.com/webhook"
}
HTTP 201
[Asserts]
jsonpath "$.secret" matches "^[0-9a-f]{64}$"


GET http://localhost:4000/webhooks
X-API-Key: test-zhq-key
HTTP 200

[Captures]
webhook_id: jsonpath "$[?(@.url=='https://example.com/webhook')].id"


DELETE http://localhost:4000/webhooks/{{webhook_id}}
X-API-Key: test-zhq-key
HTTP 204


GET http://localhost:4000/webhooks
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$" count == 0
