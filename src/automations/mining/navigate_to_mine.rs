use super::try_refuel;
use super::wait_until;
use super::State;
use crate::queries::Query;
use chrono::{DateTime, Utc};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn navigate_to_mine(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
    arrival: DateTime<Utc>,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    wait_until(arrival).await?;
    let _ = try_refuel(client, sender, token, ship_symbol).await?;
    Ok(State::Mining)
}
