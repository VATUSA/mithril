# Pagination, facility-visibility, and start-date-filtering tests for
# GET /events.
#
# Runs after events.hurl (alphabetical glob order) which cleans up fully, and
# before facility.hurl / testing_key.hurl, the latter of which asserts
# GET /events returns count == 0 -- so this file must delete every event it
# creates before finishing, leaving the table empty again.
#
# Events 1-4 all land on the same UTC day (2033-05-18) and drive the
# pagination/visibility assertions below. Events 5-7 are added afterward,
# one per UTC calendar day (2033-05-18, -19, -20), to exercise
# start_date_from/start_date_to as day boundaries.

POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 1",
    "body": "Seeded for pagination tests",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 2000000000,
    "end_time": 2000003600,
    "created_by": 1000001
}
HTTP 201

POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 2",
    "body": "Seeded for pagination tests",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 2000000200,
    "end_time": 2000003800,
    "created_by": 1000001
}
HTTP 201

POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 3",
    "body": "Seeded for pagination tests",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 2000000100,
    "end_time": 2000003700,
    "created_by": 1000001
}
HTTP 201

# Owned by ZHU (a ZHQ key can override the facility on create). Like the
# other three, review_status stays NULL (unapproved), so this event should
# only be visible to ZHQ keys and to a ZHU-facility key -- not to anonymous
# callers or other facilities.
POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 4",
    "body": "Seeded for pagination tests",
    "banner_image_url": "https://example.com/banner.png",
    "facility": "ZHU",
    "start_time": 2000000300,
    "end_time": 2000003900,
    "created_by": 1000001
}
HTTP 201


GET http://localhost:4000/events
X-API-Key: test-zhq-key
HTTP 200

[Captures]
id1: jsonpath "$[?(@.title=='Pagination test event 1')].id"
id2: jsonpath "$[?(@.title=='Pagination test event 2')].id"
id3: jsonpath "$[?(@.title=='Pagination test event 3')].id"
id4: jsonpath "$[?(@.title=='Pagination test event 4')].id"
[Asserts]
jsonpath "$" count == 4
header "X-Total-Count" == "4"
header "X-Total-Pages" == "1"


# Page 1 of 2, ordered by start_time ascending (not creation order): event 1
# (2000000000) then event 3 (2000000100).
GET http://localhost:4000/events?page=1&count=2
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "4"
header "X-Total-Pages" == "2"
jsonpath "$" count == 2
jsonpath "$[0].title" == "Pagination test event 1"
jsonpath "$[1].title" == "Pagination test event 3"


# Page 2 of 2: event 2 (2000000200) then event 4 (2000000300).
GET http://localhost:4000/events?page=2&count=2
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "4"
header "X-Total-Pages" == "2"
jsonpath "$" count == 2
jsonpath "$[0].title" == "Pagination test event 2"
jsonpath "$[1].title" == "Pagination test event 4"


# Past the last page: empty body, headers still reflect the full total.
GET http://localhost:4000/events?page=99&count=2
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "4"
header "X-Total-Pages" == "2"
jsonpath "$" count == 0


# count is clamped to a minimum of 1, not rejected or treated as "all".
GET http://localhost:4000/events?count=0
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$" count == 1
jsonpath "$[0].title" == "Pagination test event 1"


# count is capped at 100; with only 4 events the body is unaffected, but the
# request must still succeed rather than error.
GET http://localhost:4000/events?count=1000
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$" count == 4


# None of these events have been approved, so an anonymous caller sees none
# of them despite the ZHQ key seeing all four.
GET http://localhost:4000/events
HTTP 200
[Asserts]
header "X-Total-Count" == "0"
header "X-Total-Pages" == "1"
jsonpath "$" count == 0


# A facility-scoped key only sees its own facility's events (event 4, owned
# by ZHU) plus any approved events (none here) -- not ZHQ's other events.
GET http://localhost:4000/events
X-API-Key: test-facility-key
HTTP 200
[Asserts]
header "X-Total-Count" == "1"
header "X-Total-Pages" == "1"
jsonpath "$" count == 1
jsonpath "$[0].title" == "Pagination test event 4"


# Three more events, one per UTC calendar day (2033-05-18, -19, -20), to
# exercise start_date_from/start_date_to as day boundaries. Event 1-4 all
# land on 2033-05-18, so event 5 joins them there while 6 and 7 land on the
# following two days.
POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 5",
    "body": "Seeded for start-date-filter tests",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 2000030400,
    "end_time": 2000034000,
    "created_by": 1000001
}
HTTP 201

POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 6",
    "body": "Seeded for start-date-filter tests",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 2000116800,
    "end_time": 2000120400,
    "created_by": 1000001
}
HTTP 201

POST http://localhost:4000/events
X-API-Key: test-zhq-key
Content-Type: application/json
{
    "title": "Pagination test event 7",
    "body": "Seeded for start-date-filter tests",
    "banner_image_url": "https://example.com/banner.png",
    "start_time": 2000203200,
    "end_time": 2000206800,
    "created_by": 1000001
}
HTTP 201


GET http://localhost:4000/events
X-API-Key: test-zhq-key
HTTP 200
[Captures]
id5: jsonpath "$[?(@.title=='Pagination test event 5')].id"
id6: jsonpath "$[?(@.title=='Pagination test event 6')].id"
id7: jsonpath "$[?(@.title=='Pagination test event 7')].id"
[Asserts]
jsonpath "$" count == 7


# start_date_from only: excludes events 1-5 (05-18), includes 6 (05-19) and
# 7 (05-20).
GET http://localhost:4000/events?start_date_from=2033-05-19
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "2"
jsonpath "$" count == 2
jsonpath "$[0].title" == "Pagination test event 6"
jsonpath "$[1].title" == "Pagination test event 7"


# start_date_to only: inclusive of the whole 05-19 day, so it excludes event
# 7 (05-20) but includes events 1-5 (05-18) and 6 (05-19).
GET http://localhost:4000/events?start_date_to=2033-05-19
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "6"
jsonpath "$" count == 6


# Combined range narrows to a single day/event.
GET http://localhost:4000/events?start_date_from=2033-05-19&start_date_to=2033-05-19
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "1"
jsonpath "$" count == 1
jsonpath "$[0].title" == "Pagination test event 6"


# Range with no matches.
GET http://localhost:4000/events?start_date_from=2033-05-21
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
header "X-Total-Count" == "0"
jsonpath "$" count == 0


# Malformed dates are rejected outright.
GET http://localhost:4000/events?start_date_from=not-a-date
X-API-Key: test-zhq-key
HTTP 400

GET http://localhost:4000/events?start_date_to=2033-13-40
X-API-Key: test-zhq-key
HTTP 400


# start_date_from after start_date_to is rejected.
GET http://localhost:4000/events?start_date_from=2033-05-20&start_date_to=2033-05-18
X-API-Key: test-zhq-key
HTTP 400


# Clean up so later tests (testing_key.hurl asserts GET /events count == 0)
# see an empty table again.
DELETE http://localhost:4000/events/{{id1}}
X-API-Key: test-zhq-key
HTTP 204

DELETE http://localhost:4000/events/{{id2}}
X-API-Key: test-zhq-key
HTTP 204

DELETE http://localhost:4000/events/{{id3}}
X-API-Key: test-zhq-key
HTTP 204

DELETE http://localhost:4000/events/{{id4}}
X-API-Key: test-zhq-key
HTTP 204

DELETE http://localhost:4000/events/{{id5}}
X-API-Key: test-zhq-key
HTTP 204

DELETE http://localhost:4000/events/{{id6}}
X-API-Key: test-zhq-key
HTTP 204

DELETE http://localhost:4000/events/{{id7}}
X-API-Key: test-zhq-key
HTTP 204


GET http://localhost:4000/events
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$" count == 0
