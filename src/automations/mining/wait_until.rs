use chrono::{DateTime, Utc};

pub async fn wait_until(
    arrival: DateTime<Utc>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let time = Utc::now();
    if arrival > time {
        let duration = arrival - time;
        tokio::time::sleep(duration.to_std()?).await;
    }
    Ok(())
}
