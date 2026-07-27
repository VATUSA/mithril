# CRUD lifecycle for /events: create -> read -> update -> read -> delete -> read
# Run against a freshly-seeded stack; see docker-compose.test.yml.

POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Integration test event",
    "body": "Created by hurl",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 1893456000,
    "end_time": 1893459600,
    "created_by": 1000001
}
HTTP 201

GET http://localhost:4000/events
X-API-Key: test-zhq-key
HTTP 200

[Captures]
event_id: jsonpath "$[?(@.title=='Integration test event')].id"


GET http://localhost:4000/events/{{event_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.title" == "Integration test event"
jsonpath "$.body" == "Created by hurl"
jsonpath "$.facility" == "ZHQ"


PATCH http://localhost:4000/events/{{event_id}}
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Integration test event (updated)",
    "body": "Updated by hurl",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 1893456000,
    "end_time": 1893459600,
    "updated_by": 1000001
}
HTTP 200


GET http://localhost:4000/events/{{event_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.title" == "Integration test event (updated)"
jsonpath "$.body" == "Updated by hurl"


DELETE http://localhost:4000/events/{{event_id}}
X-API-Key: test-zhq-key
HTTP 204


GET http://localhost:4000/events/{{event_id}}
X-API-Key: test-zhq-key
HTTP 404
