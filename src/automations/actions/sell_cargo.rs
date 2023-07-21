use crate::automations::utilities::find_delivery_in_contracts;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::Cargo;
use log::info;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn sell_cargo(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<Cargo, Box<dyn std::error::Error + Send + Sync>> {
    let cargo_response = queries::cargo(client, sender, token, ship_symbol).await?;
    let contracts_response = queries::contracts(client, sender, token).await?;

    let _ = queries::dock(client, sender, token, ship_symbol).await?;

    let mut cargo = cargo_response.clone();
    for item in cargo_response.inventory.iter() {
        if find_delivery_in_contracts(&contracts_response, &item.symbol).is_some() {
            continue;
        }

        let sell_response =
            queries::sell(client, sender, token, ship_symbol, &item.symbol, item.units).await?;
        info!(
            "Ship {ship_symbol} sold {} units of {} for {} credits",
            sell_response.transaction.units,
            sell_response.transaction.trade_symbol,
            sell_response.transaction.total_price
        );
        cargo = sell_response.cargo;
    }

    Ok(cargo)
}
