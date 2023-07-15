use super::State;
use crate::queries;
use crate::queries::Query;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn refuel_ship(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let _ = queries::dock(client, sender, token, ship_symbol).await?;
    let _ = queries::refuel(client, sender, token, ship_symbol).await?;
    Ok(State::LookingForMine)
}
