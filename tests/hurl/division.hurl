# Read-only test for /division/controllers against the seeded controller and
# role grants in 03_seed.sql. The seeded ZZN and ZAE users must not appear.

GET http://localhost:4000/division/controllers
HTTP 401

GET http://localhost:4000/division/controllers
X-API-Key: test-facility-key
HTTP 200
[Asserts]
jsonpath "$.controllers" count == 1
jsonpath "$.controllers[0].cid" == 1000001
jsonpath "$.controllers[0].display_name" == "Test Controller"
jsonpath "$.controllers[0].controller_rating" == 5
jsonpath "$.controllers[0].facility" == "ZHU"
jsonpath "$.controllers[0].visiting_facilities" count == 1
jsonpath "$.controllers[0].visiting_facilities[0]" == "ZJX"
jsonpath "$.roles" count == 2
jsonpath "$.roles[0].cid" == 1000001
jsonpath "$.roles[0].facility" == "ZHU"
jsonpath "$.roles[0].role" == "mentor"
jsonpath "$.roles[1].facility" == "*"
jsonpath "$.roles[1].role" == "ace_team"
