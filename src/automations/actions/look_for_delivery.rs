use crate::automations::utilities::DeliveryNotInSystemError;
use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::Delivery;
use chrono::{DateTime, Utc};
use log::{info, warn};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn look_for_delivery(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
    delivery: &Delivery,
) -> Result<Option<DateTime<Utc>>, Box<dyn std::error::Error + Send + Sync>> {
    info!(
        "Ship {ship_symbol} found delivery for {} in waypoint {}",
        delivery.trade_symbol, delivery.destination_symbol
    );
    let orbit_response = queries::orbit(client, sender, token, ship_symbol).await?;

    if orbit_response.nav.waypoint_symbol == delivery.destination_symbol {
        return Ok(None);
    }

    if orbit_response.nav.system_symbol == delivery.destination_symbol.system() {
        let navigate_response = queries::navigate(
            client,
            sender,
            token,
            ship_symbol,
            &delivery.destination_symbol,
        )
        .await?;
        Ok(Some(navigate_response.nav.route.arrival))
    } else {
        warn!(
            "Ship {} is in system {}, but delivery for trade good {} is in system {}",
            ship_symbol.to_owned(),
            orbit_response.nav.system_symbol,
            delivery.trade_symbol,
            delivery.destination_symbol.system()
        );
        Err(DeliveryNotInSystemError {
            ship_symbol: ship_symbol.to_owned(),
            trade_good: delivery.trade_symbol.clone(),
            ship_system: orbit_response.nav.system_symbol,
            trade_good_system: delivery.destination_symbol.system(),
        }
        .into())
    }
}
