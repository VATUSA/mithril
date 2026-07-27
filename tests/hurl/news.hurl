# CRUD lifecycle for /news: create -> read -> update -> read -> delete -> read
# Run against a freshly-seeded stack; see docker-compose.test.yml.

POST http://localhost:4000/news
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Integration test post",
    "body": "Created by hurl",
    "author_cid": 1000001
}
HTTP 201

GET http://localhost:4000/news
X-API-Key: test-zhq-key
HTTP 200

[Captures]
news_id: jsonpath "$[?(@.title=='Integration test post')].id"


GET http://localhost:4000/news/{{news_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.title" == "Integration test post"
jsonpath "$.body" == "Created by hurl"
jsonpath "$.author_cid" == 1000001


PATCH http://localhost:4000/news/{{news_id}}
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Integration test post (updated)",
    "body": "Updated by hurl"
}
HTTP 200


GET http://localhost:4000/news/{{news_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.title" == "Integration test post (updated)"
jsonpath "$.body" == "Updated by hurl"


DELETE http://localhost:4000/news/{{news_id}}
X-API-Key: test-zhq-key
HTTP 204


GET http://localhost:4000/news/{{news_id}}
X-API-Key: test-zhq-key
HTTP 404
