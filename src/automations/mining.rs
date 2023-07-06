use crate::queries::Query;
use crate::{local_data::Credentials, queries};
use log::{info, trace};
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::time;

pub async fn mining(
    client: Client,
    sender: Sender<Query>,
    credentials: Credentials,
    ship_symbol: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Started mining task for ship {ship_symbol}");
    let mut interval = time::interval(time::Duration::from_secs(30));

    loop {
        interval.tick().await;
        trace!("Running mining task for ship {ship_symbol}");

        let ship_response =
            queries::ship(&client, &sender, &credentials.token, &ship_symbol).await?;

        let cargo_capacity = ship_response.cargo.capacity;
        let cargo_units = ship_response.cargo.units;
        let status = ship_response.nav.status;
        let system = ship_response.nav.system_symbol;
        let waypoint = ship_response.nav.waypoint_symbol;
        let waypoint_response = queries::waypoint(
            &client,
            &sender,
            credentials.token.as_str(),
            &system,
            &waypoint,
        )
        .await?;

        let state = if cargo_units == cargo_capacity {
            State::Selling
        } else {
            State::Mining
        };
    }
}

enum State {
    Mining,
    NavigatingToMarket,
    Selling,
    NavigatingToFuel,
    Refuelling,
    NavigatingToMine,
    OutOfFuel,
}
