use super::State;
use crate::automations::utilities::find_delivery_in_contracts;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::{Cargo, Fuel};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn state_after_delivering(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
    cargo: Cargo,
    refuelled: Option<Fuel>,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let contracts_response = queries::contracts(client, sender, token).await?;
    let nav_response = queries::nav(client, sender, token, ship_symbol).await?;

    // If contract items left, go sell them. If here, sell them first
    for item in cargo.inventory.iter() {
        if let Some((new_contract, new_delivery)) =
            find_delivery_in_contracts(&contracts_response, &item.symbol)
        {
            if new_delivery.destination_symbol != nav_response.waypoint_symbol {
                return Ok(State::LookingForDelivery {
                    contract: new_contract.clone(),
                    delivery: new_delivery.clone(),
                    units: item.units,
                });
            }
        }
    }

    match refuelled {
        Some(_) => Ok(State::LookingForMine),
        None => Ok(State::LookingForFuel),
    }
}
