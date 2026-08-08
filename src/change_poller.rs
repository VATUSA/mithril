//! Background poller for the `roster_notifications` table.

use crate::db::{ChangeLogEntry, Webhook};
use crate::queries;
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;
use sqlx::MySqlPool;
use std::collections::HashSet;
use std::time::Duration;
use tokio::time::interval;

const RETENTION_DAYS: u32 = 7;
const DELIVERY_TIMEOUT: Duration = Duration::from_secs(5);

/// Poll `roster_notifications` every 15 seconds and handle rows.
/// Once an hour, delete processed rows older than [`RETENTION_DAYS`].
///
/// `vatusa_db` holds `roster_notifications`; `cobalt_db` holds `v3_webhook`.
pub async fn run(
    vatusa_db: MySqlPool,
    cobalt_db: MySqlPool,
    shutdown: impl std::future::Future<Output = ()>,
) {
    let http = reqwest::Client::builder()
        .timeout(DELIVERY_TIMEOUT)
        .build()
        .expect("failed to build webhook delivery HTTP client");

    let mut ticker = interval(Duration::from_secs(15));
    let mut cleanup_ticker = interval(Duration::from_hours(1));
    tokio::pin!(shutdown);

    // sleep for startup, but stay interruptible: a shutdown arriving during
    // this window must not be deferred until the sleep elapses.
    tokio::select! {
        () = tokio::time::sleep(Duration::from_secs(5)) => {}
        () = &mut shutdown => {
            tracing::warn!("change_poller shutting down during startup");
            return;
        }
    }

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if let Err(e) = poll_once(&vatusa_db, &cobalt_db, &http).await {
                    tracing::error!("change_poller error: {e}");
                }
            }
            _ = cleanup_ticker.tick() => {
                if let Err(e) = cleanup_once(&vatusa_db).await {
                    tracing::error!("change_poller cleanup error: {e}");
                }
            }
            () = &mut shutdown => {
                tracing::warn!("change_poller shutting down");
                break;
            }
        }
    }
}

/// Check the `roster_notifications` DB table for unprocessed rows and deliver
/// matching webhooks.
async fn poll_once(
    vatusa_db: &MySqlPool,
    cobalt_db: &MySqlPool,
    http: &reqwest::Client,
) -> Result<(), crate::shared::AppError> {
    let changes = queries::claim_unprocessed_changes(vatusa_db, 100).await?;
    for change in changes {
        tracing::debug!(
            "[roster_notifications #{}] {} {} pk={} old={} new={}",
            change.id,
            change.table_name,
            change.operation,
            change.row_pk,
            change
                .old_value
                .as_ref()
                .map_or_else(|| "null".into(), std::string::ToString::to_string),
            change
                .new_value
                .as_ref()
                .map_or_else(|| "null".into(), std::string::ToString::to_string)
        );

        for facility in target_facilities(&change) {
            let webhooks = queries::get_webhooks_for_facility_full(cobalt_db, &facility).await?;
            for webhook in webhooks {
                deliver(http, &webhook, &change).await;
            }
        }
    }
    Ok(())
}

/// Determine which facilities should be notified of a change.
///
/// For `controllers` rows, the trigger only logs a `facility` change, so both
/// the old (losing) and new (gaining) facility are notified when they
/// differ. For `visits` rows, the facility is embedded in whichever of
/// `new_value`/`old_value` is present (`new_value` for inserts/updates,
/// `old_value` for deletes).
fn target_facilities(change: &ChangeLogEntry) -> HashSet<String> {
    let mut facilities = HashSet::new();
    let mut add_facility = |value: &Option<serde_json::Value>| {
        if let Some(facility) = value
            .as_ref()
            .and_then(|v| v.get("facility"))
            .and_then(|f| f.as_str())
        {
            facilities.insert(facility.to_string());
        }
    };
    add_facility(&change.old_value);
    add_facility(&change.new_value);
    facilities
}

/// Sign and deliver a single change payload to a single webhook.
///
/// This is a single delivery attempt with no retries; failures are logged
/// and otherwise ignored.
async fn deliver(http: &reqwest::Client, webhook: &Webhook, change: &ChangeLogEntry) {
    let payload = serde_json::json!({
        "type": "roster_change",
        "data": {
            "id": change.id,
            "table_name": change.table_name,
            "operation": change.operation,
            "row_pk": change.row_pk,
            "old_value": change.old_value,
            "new_value": change.new_value,
            "created_at": change.created_at,
        },
    });
    let body = match serde_json::to_vec(&payload) {
        Ok(body) => body,
        Err(e) => {
            tracing::error!("failed to serialize webhook payload: {e}");
            return;
        }
    };

    let mut mac = match Hmac::<Sha256>::new_from_slice(webhook.secret.as_bytes()) {
        Ok(mac) => mac,
        Err(e) => {
            tracing::error!(
                "failed to build webhook HMAC for webhook {}: {e}",
                webhook.id
            );
            return;
        }
    };
    mac.update(&body);
    let signature = hex::encode(mac.finalize().into_bytes());

    let result = http
        .post(&webhook.url)
        .header("Content-Type", "application/json")
        .header("X-Mithril-Signature", format!("sha256={signature}"))
        .body(body)
        .send()
        .await;

    match result {
        Ok(resp) if resp.status().is_success() => {
            tracing::debug!("delivered webhook {} to {}", webhook.id, webhook.url);
        }
        Ok(resp) => {
            tracing::warn!(
                "webhook {} delivery to {} failed with status {}",
                webhook.id,
                webhook.url,
                resp.status()
            );
        }
        Err(e) => {
            tracing::warn!(
                "webhook {} delivery to {} failed: {e}",
                webhook.id,
                webhook.url
            );
        }
    }
}

/// Delete processed `roster_notifications` rows past their retention window.
async fn cleanup_once(db: &MySqlPool) -> Result<(), crate::shared::AppError> {
    let deleted = queries::delete_processed_changes(db, RETENTION_DAYS).await?;
    if deleted > 0 {
        tracing::debug!("change_poller cleanup: deleted {deleted} processed row(s)");
    }
    Ok(())
}
