# Webhooks

Mithril can deliver outbound webhooks to notify facilities of roster changes in near
real time, as an alternative to polling the API. Delivery is currently a **single
attempt with no retries**; if your endpoint is unreachable or errors, the event is
not redelivered.

## Creating / deleting a webhook

TBD.

## Payload shape

Every webhook delivery is a JSON object with a top-level `type` field identifying the
event, and a `data` field containing the event's payload. New event types may be
added in the future without changing this envelope, so clients should ignore `type`
values they don't recognize rather than treating them as an error.

```json
{
  "type": "roster_change",
  "data": { ... }
}
```

### `roster_change`

Emitted for changes to the `controllers` and `visits` tables that affect a facility's
roster (e.g. a controller joining, leaving, or transferring facilities; a visiting
controller being added or removed). For a `controllers` row where the facility
changes, the event is delivered to **both** the losing and gaining facility's
webhooks.

```json
{
  "type": "roster_change",
  "data": {
    "id": 1,
    "table_name": "controllers",
    "operation": "UPDATE",
    "row_pk": 800007,
    "old_value": { "facility": "ZAE" },
    "new_value": { "facility": "ZDC" },
    "created_at": "2026-07-12T23:26:34Z"
  }
}
```

- `table_name`: the source table (`controllers` or `visits`).
- `operation`: `INSERT`, `UPDATE`, or `DELETE`.
- `row_pk`: primary key of the affected row.
- `old_value` / `new_value`: JSON snapshots of the relevant fields before/after the
  change. `old_value` is `null` for `INSERT`; `new_value` is `null` for `DELETE`.
- `created_at`: UTC timestamp the change was recorded, ISO 8601.

## Verifying deliveries

Each delivery includes an `X-Mithril-Signature` header of the form:

```
X-Mithril-Signature: sha256=<hex-encoded HMAC-SHA256 digest>
```

The digest is computed over the raw JSON request body, keyed with the webhook's
secret (returned once, at creation time — it cannot be retrieved again). Recipients
should recompute this HMAC over the raw body they received and compare it to the
header using a constant-time comparison, rejecting the request if they don't match.

Example verification in Python (Flask):

```python
import hashlib
import hmac

def verify(secret: str, body: bytes, signature_header: str) -> bool:
    expected = hmac.new(secret.encode(), body, hashlib.sha256).hexdigest()
    return hmac.compare_digest(f"sha256={expected}", signature_header)
```

## Delivery details

- Requests are `POST`ed as `Content-Type: application/json`.
- A 5-second timeout is applied per delivery attempt.
- Any non-2xx response or request error is logged server-side and otherwise ignored;
  there is no retry or dead-lettering yet (planned future improvement).
