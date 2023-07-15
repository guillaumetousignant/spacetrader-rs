use super::wait_until;
use super::State;
use chrono::{DateTime, Utc};

pub async fn navigate_to_fuel(
    arrival: DateTime<Utc>,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    wait_until(arrival).await?;
    Ok(State::Refuelling)
}
