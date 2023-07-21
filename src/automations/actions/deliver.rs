use crate::automations::utilities::find_delivery_in_contracts;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::{Cargo, Contract, Delivery};
use log::info;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn deliver(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
    contract: Contract,
    delivery: Delivery,
    units: u128,
) -> Result<Cargo, Box<dyn std::error::Error + Send + Sync>> {
    let _ = queries::dock(client, sender, token, ship_symbol).await?;

    let mut deliver_response = queries::deliver(
        client,
        sender,
        token,
        &contract.id,
        ship_symbol,
        &delivery.trade_symbol,
        units,
    )
    .await?;

    info!(
        "Ship {ship_symbol} delivered {} units of {} for contract {}",
        units, delivery.trade_symbol, deliver_response.contract.id
    );

    // Any other items to deliver here?
    let cargo_response = queries::cargo(client, sender, token, ship_symbol).await?;
    let contracts_response = queries::contracts(client, sender, token).await?;

    for item in cargo_response.inventory.iter() {
        if let Some((other_contract, other_delivery)) =
            find_delivery_in_contracts(&contracts_response, &item.symbol)
        {
            if other_delivery.destination_symbol == delivery.destination_symbol {
                deliver_response = queries::deliver(
                    client,
                    sender,
                    token,
                    &other_contract.id,
                    ship_symbol,
                    &other_delivery.trade_symbol,
                    item.units,
                )
                .await?;

                info!(
                    "Ship {ship_symbol} delivered {} units of {} for contract {}",
                    item.units, other_delivery.trade_symbol, other_contract.id
                );
            }
        }
    }

    Ok(deliver_response.cargo)
}
