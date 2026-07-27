# Read-only smoke test for /facility. Only get_facilities and get_facility_info
# are implemented (the rest of routes/facility.rs is unimplemented stubs), and
# the schema-only fixtures don't seed any facility rows, so this just checks
# the endpoints respond correctly against an empty vatusa_old facilities table.

GET http://localhost:4000/facility
HTTP 200
[Asserts]
jsonpath "$" count == 0

GET http://localhost:4000/facility/ZHQ
HTTP 404
