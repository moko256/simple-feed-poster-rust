use chrono::{DateTime, Utc};
use tokio::time::sleep;

pub async fn sleep_at(time: DateTime<Utc>) {
    let until = time - Utc::now();
    if let Ok(until) = until.to_std() {
        sleep(until).await;
    }
}
