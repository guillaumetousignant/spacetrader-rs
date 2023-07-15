use super::find_delivery;
use super::State;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::Contract;
use log::trace;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn sell_cargo(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let cargo_response = queries::cargo(client, sender, token, ship_symbol).await?;
    let contracts_response = queries::contracts(client, sender, token).await?;

    let _ = queries::dock(client, sender, token, ship_symbol).await?;

    for item in cargo_response.inventory.iter() {
        if find_in_contracts(&contracts_response, &item.symbol) {
            continue;
        }

        trace!(
            "Ship {ship_symbol} selling {} units of {}",
            item.symbol,
            item.units
        );
        let _ = queries::sell(client, sender, token, ship_symbol, &item.symbol, item.units).await?;
    }

    Ok(State::LookingForFuel) // Unless ship is full, then State::LookingForMine
                              // Actually, go to contract instead, and contract can do this
}

fn find_in_contracts(contracts: &[Contract], trade_good: &str) -> bool {
    for contract in contracts.iter() {
        if find_delivery(&contract.terms.deliver, trade_good).is_some() {
            return true;
        }
    }
    return false;
}
