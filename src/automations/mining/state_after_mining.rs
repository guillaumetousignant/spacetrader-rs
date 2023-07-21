use super::State;
use crate::automations::utilities::find_delivery_in_contracts;
use crate::automations::utilities::{InvalidNextTargetError, TradeGoodNotFoundInContractsError};
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::{Cargo, Fuel};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn state_after_mining(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
    cargo: Cargo,
    refuelled: Option<Fuel>,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let contracts_response = queries::contracts(client, sender, token).await?;
    for item in cargo.inventory.iter() {
        if find_delivery_in_contracts(&contracts_response, &item.symbol).is_none() {
            return Ok(State::LookingForMarket);
        }
    }

    match cargo.inventory.first() {
        Some(item) => {
            let (contract, delivery) =
                find_delivery_in_contracts(&contracts_response, &item.symbol).ok_or(
                    TradeGoodNotFoundInContractsError {
                        ship_symbol: ship_symbol.to_owned(),
                        trade_good: item.symbol.clone(),
                    },
                )?;
            Ok(State::LookingForDelivery {
                contract: contract.clone(),
                delivery: delivery.clone(),
                units: item.units,
            })
        }
        // Should not really get here, it means that the cargo is full, but there is no non-contract items and no contract items
        None => match refuelled {
            Some(_) => Err(InvalidNextTargetError {
                ship_symbol: ship_symbol.to_owned(),
            }
            .into()),

            None => Ok(State::LookingForFuel),
        },
    }
}
