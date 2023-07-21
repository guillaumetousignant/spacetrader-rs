use super::WAIT_UNTIL_BUFFER_MILLIS;
use chrono::{DateTime, Duration, Utc};

pub async fn wait_until(
    arrival: DateTime<Utc>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let adjusted_arrival = arrival + Duration::milliseconds(WAIT_UNTIL_BUFFER_MILLIS);
    let time = Utc::now();
    if adjusted_arrival > time {
        let duration = adjusted_arrival - time;
        tokio::time::sleep(duration.to_std()?).await;
    }
    Ok(())
}
