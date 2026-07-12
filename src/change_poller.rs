//! Background poller for the `roster_notifications` table.

use crate::queries::{delete_processed_changes, get_unprocessed_changes, mark_change_processed};
use sqlx::MySqlPool;
use std::time::Duration;
use tokio::time::interval;

const RETENTION_DAYS: u32 = 7;

/// Poll `roster_notifications` every 15 seconds and handle rows.
/// Once an hour, delete processed rows older than [`RETENTION_DAYS`].
pub async fn run(db: MySqlPool, shutdown: impl std::future::Future<Output = ()>) {
    let mut ticker = interval(Duration::from_secs(15));
    let mut cleanup_ticker = interval(Duration::from_secs(60 * 60));
    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                if let Err(e) = poll_once(&db).await {
                    tracing::error!("change_poller error: {e}");
                }
            }
            _ = cleanup_ticker.tick() => {
                if let Err(e) = cleanup_once(&db).await {
                    tracing::error!("change_poller cleanup error: {e}");
                }
            }
            _ = &mut shutdown => {
                tracing::warn!("change_poller shutting down");
                break;
            }
        }
    }
}

/// Check the `roster_notifications` DB table for unprocessed rows.
async fn poll_once(db: &MySqlPool) -> Result<(), crate::shared::AppError> {
    let changes = get_unprocessed_changes(db, 100).await?;
    for change in changes {
        // tracing::info!(
        //     "[roster_notifications #{}] {} {} pk={} old={} new={}",
        //     change.id,
        //     change.table_name,
        //     change.operation,
        //     change.row_pk,
        //     change
        //         .old_value
        //         .as_ref()
        //         .map(|v| v.to_string())
        //         .unwrap_or_else(|| "null".into()),
        //     change
        //         .new_value
        //         .as_ref()
        //         .map(|v| v.to_string())
        //         .unwrap_or_else(|| "null".into()),
        // );
        // TODO
        mark_change_processed(db, change.id).await?;
    }
    Ok(())
}

/// Delete processed `roster_notifications` rows past their retention window.
async fn cleanup_once(db: &MySqlPool) -> Result<(), crate::shared::AppError> {
    let deleted = delete_processed_changes(db, RETENTION_DAYS).await?;
    if deleted > 0 {
        tracing::info!("change_poller cleanup: deleted {deleted} processed row(s)");
    }
    Ok(())
}
