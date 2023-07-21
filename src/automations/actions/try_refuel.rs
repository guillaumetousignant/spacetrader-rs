use crate::automations::utilities::find_trade_good;
use crate::automations::utilities::find_trait;
use crate::automations::utilities::{FUEL_SYMBOL, MARKET_TRAIT};
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::Fuel;
use log::info;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn try_refuel(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<Option<Fuel>, Box<dyn std::error::Error + Send + Sync>> {
    let ship_response = queries::ship(client, sender, token, ship_symbol).await?;
    let waypoint_response = queries::waypoint(
        client,
        sender,
        token,
        &ship_response.nav.system_symbol,
        &ship_response.nav.waypoint_symbol,
    )
    .await?;

    let marketplace_trait = find_trait(&waypoint_response.traits, MARKET_TRAIT); // CHECK will only find the first market

    match marketplace_trait {
        Some(_) => {
            let marketplace_response = queries::market(
                client,
                sender,
                token,
                &waypoint_response.system_symbol,
                &waypoint_response.symbol,
            )
            .await?;

            let fuel_trade_good = find_trade_good(&marketplace_response.trade_goods, FUEL_SYMBOL);

            match fuel_trade_good {
                Some(_) => {
                    info!(
                        "Ship {ship_symbol} successfully refuelling in waypoint {}",
                        ship_response.nav.waypoint_symbol
                    );
                    let _ = queries::dock(client, sender, token, ship_symbol).await?;
                    let refuel_response =
                        queries::refuel(client, sender, token, ship_symbol).await?;
                    Ok(Some(refuel_response.fuel))
                }
                None => Ok(None),
            }
        }
        None => Ok(None),
    }
}
