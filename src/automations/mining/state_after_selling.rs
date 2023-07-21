use super::State;
use crate::automations::utilities::find_delivery_in_contracts;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::{Cargo, Fuel};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn state_after_selling(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    cargo: Cargo,
    refuelled: Option<Fuel>,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    // We only delivery if we have a full cargo of deliverable goods (which should not happen here)
    if cargo.units >= cargo.capacity {
        let contracts_response = queries::contracts(client, sender, token).await?;

        for item in cargo.inventory {
            if let Some((contract, delivery)) =
                find_delivery_in_contracts(&contracts_response, &item.symbol)
            {
                return Ok(State::LookingForDelivery {
                    contract: contract.clone(),
                    delivery: delivery.clone(),
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
