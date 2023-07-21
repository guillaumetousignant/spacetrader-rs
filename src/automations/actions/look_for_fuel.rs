use crate::automations::utilities::find_trade_good;
use crate::automations::utilities::find_trade_good_in_system;
use crate::automations::utilities::find_trait;
use crate::automations::utilities::TradeGoodNotFoundInSystemError;
use crate::automations::utilities::{FUEL_SYMBOL, MARKET_TRAIT};
use crate::queries;
use crate::queries::Query;
use chrono::{DateTime, Utc};
use log::{info, warn};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn look_for_fuel(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<Option<DateTime<Utc>>, Box<dyn std::error::Error + Send + Sync>> {
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
    if let Some(_) = marketplace_trait {
        let marketplace_response = queries::market(
            client,
            sender,
            token,
            &waypoint_response.system_symbol,
            &waypoint_response.symbol,
        )
        .await?;

        let fuel_trade_good = find_trade_good(&marketplace_response.trade_goods, FUEL_SYMBOL);

        if let Some(_) = fuel_trade_good {
            return Ok(None);
        }
    }

    let destination = find_trade_good_in_system(
        client,
        sender,
        token,
        FUEL_SYMBOL,
        &ship_response.nav.system_symbol,
    )
    .await?;

    if let Some(dest) = destination {
        info!("Ship {ship_symbol} found {FUEL_SYMBOL} in waypoint {dest}");
        let _ = queries::orbit(client, sender, token, ship_symbol).await?;
        let navigate_response =
            queries::navigate(client, sender, token, ship_symbol, &dest).await?;
        Ok(Some(navigate_response.nav.route.arrival))
    } else {
        warn!(
            "Ship {ship_symbol} found no trade good {FUEL_SYMBOL} in system {}",
            ship_response.nav.system_symbol
        );
        Err(TradeGoodNotFoundInSystemError {
            ship_symbol: String::from(ship_symbol),
            trade_good: String::from(FUEL_SYMBOL),
            system: ship_response.nav.system_symbol,
        }
        .into())
    }
}
