# Event banner image upload (multipart) coverage: a valid upload, the
# size/format/aspect-ratio rejections, the "omitting the banner keeps the
# existing one" edit semantics, and replacing an existing banner with a new
# upload. Run against a freshly-seeded stack with MinIO standing in for
# DigitalOcean Spaces; see docker-compose.test.yml. Every event created here
# is deleted before the file finishes, since testing_key.hurl asserts GET
# /events returns count == 0 and relies on the table being empty by the time
# it runs.

# --- happy path: multipart create with a valid 16:9 banner ---
POST http://localhost:4000/events
X-API-Key: test-zhq-key
[Multipart]
title: Multipart banner event
body: Created by hurl with an uploaded banner
start_time: 1893456000
end_time: 1893459600
created_by: 1000001
banner_image: file,fixtures/banner-16x9.png;
HTTP 201

GET http://localhost:4000/events
X-API-Key: test-zhq-key
HTTP 200

[Captures]
banner_event_id: jsonpath "$[?(@.title=='Multipart banner event')].id"


GET http://localhost:4000/events/{{banner_event_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.banner_image_url" startsWith "http://localhost:9000/vatusa-events-test/event-banners/zhq/"
jsonpath "$.banner_image_url" endsWith ".png"

[Captures]
original_banner_url: jsonpath "$.banner_image_url"


# --- edit without a new file or banner_image_url: existing banner is kept ---
PATCH http://localhost:4000/events/{{banner_event_id}}
X-API-Key: test-zhq-key
[Multipart]
title: Multipart banner event (edited)
body: Updated by hurl, no new banner
start_time: 1893456000
end_time: 1893459600
updated_by: 1000001
HTTP 200

GET http://localhost:4000/events/{{banner_event_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.title" == "Multipart banner event (edited)"
jsonpath "$.banner_image_url" == "{{original_banner_url}}"


# --- edit with a new file: banner is replaced ---
PATCH http://localhost:4000/events/{{banner_event_id}}
X-API-Key: test-zhq-key
[Multipart]
title: Multipart banner event (new banner)
body: Updated by hurl with a new banner
start_time: 1893456000
end_time: 1893459600
updated_by: 1000001
banner_image: file,fixtures/banner-16x9.png;
HTTP 200

GET http://localhost:4000/events/{{banner_event_id}}
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$.banner_image_url" startsWith "http://localhost:9000/vatusa-events-test/event-banners/zhq/"
jsonpath "$.banner_image_url" != "{{original_banner_url}}"


DELETE http://localhost:4000/events/{{banner_event_id}}
X-API-Key: test-zhq-key
HTTP 204


# --- rejections: none of these persist an event, so nothing to clean up ---

# oversized (>8MB) file
POST http://localhost:4000/events
X-API-Key: test-zhq-key
[Multipart]
title: Oversized banner event
body: Should be rejected
start_time: 1893456000
end_time: 1893459600
created_by: 1000001
banner_image: file,fixtures/banner-oversized.bin;
HTTP 400

# wrong aspect ratio (square, not 16:9)
POST http://localhost:4000/events
X-API-Key: test-zhq-key
[Multipart]
title: Wrong aspect ratio banner event
body: Should be rejected
start_time: 1893456000
end_time: 1893459600
created_by: 1000001
banner_image: file,fixtures/banner-wrong-ratio.png;
HTTP 400

# not a real image, despite the .png extension - validates the decode-based
# check rather than trusting the client-supplied filename/content-type
POST http://localhost:4000/events
X-API-Key: test-zhq-key
[Multipart]
title: Fake image banner event
body: Should be rejected
start_time: 1893456000
end_time: 1893459600
created_by: 1000001
banner_image: file,fixtures/banner-not-image.png;
HTTP 400

# neither a file nor a banner_image_url supplied
POST http://localhost:4000/events
X-API-Key: test-zhq-key
[Multipart]
title: No banner event
body: Should be rejected
start_time: 1893456000
end_time: 1893459600
created_by: 1000001
HTTP 400


GET http://localhost:4000/events
X-API-Key: test-zhq-key
HTTP 200
[Asserts]
jsonpath "$" count == 0
