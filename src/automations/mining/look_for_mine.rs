use super::find_waypoint_type_in_system;
use super::State;
use super::WaypointTypeNotFoundError;
use super::MINING_WAYPOINT_TYPE;
use crate::queries;
use crate::queries::Query;
use log::{trace, warn};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn look_for_mine(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let ship_response = queries::ship(client, sender, token, ship_symbol).await?;
    let waypoint_response = queries::waypoint(
        client,
        sender,
        token,
        &ship_response.nav.system_symbol,
        &ship_response.nav.waypoint_symbol,
    )
    .await?;

    match waypoint_response.waypoint_type.as_str() {
        MINING_WAYPOINT_TYPE => Ok(State::Mining),
        _ => {
            let destination = find_waypoint_type_in_system(
                client,
                sender,
                token,
                MINING_WAYPOINT_TYPE,
                &ship_response.nav.system_symbol,
            )
            .await?;

            if let Some(dest) = destination {
                trace!("Ship {ship_symbol} found waypoint of type {MINING_WAYPOINT_TYPE} in waypoint {dest}");
                let _ = queries::orbit(client, sender, token, ship_symbol).await?;
                let navigate_response =
                    queries::navigate(client, sender, token, ship_symbol, &dest).await?;
                Ok(State::NavigatingToMine {
                    arrival: navigate_response.nav.route.arrival,
                })
            } else {
                warn!(
                    "Ship {ship_symbol} found no waypoint of type {MINING_WAYPOINT_TYPE} in system {}",
                    ship_response.nav.system_symbol
                );
                return Err(WaypointTypeNotFoundError {
                    ship_symbol: String::from(ship_symbol),
                    waypoint_type: String::from(MINING_WAYPOINT_TYPE),
                    system: ship_response.nav.system_symbol,
                }
                .into());
            }
        }
    }
}
