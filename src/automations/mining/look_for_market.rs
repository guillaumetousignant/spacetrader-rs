use super::State;
use super::TraitNotFoundError;
use super::MARKET_TRAIT;
use super::{find_trait, find_trait_in_system};
use crate::queries;
use crate::queries::Query;
use log::{trace, warn};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn look_for_market(
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

    let marketplace_trait = find_trait(&waypoint_response.traits, MARKET_TRAIT); // CHECK will only find the first market
    if let Some(_) = marketplace_trait {
        return Ok(State::Selling);
    }

    let destination = find_trait_in_system(
        client,
        sender,
        token,
        MARKET_TRAIT,
        &ship_response.nav.system_symbol,
    )
    .await?;

    if let Some(dest) = destination {
        trace!("Ship {ship_symbol} found trait {MARKET_TRAIT} in waypoint {dest}");
        let _ = queries::orbit(client, sender, token, ship_symbol).await?;
        let navigate_response =
            queries::navigate(client, sender, token, ship_symbol, &dest).await?;
        Ok(State::NavigatingToMarket {
            arrival: navigate_response.nav.route.arrival,
        })
    } else {
        warn!(
            "Ship {ship_symbol} found no trait {MARKET_TRAIT} in system {}",
            ship_response.nav.system_symbol
        );
        Err(TraitNotFoundError {
            ship_symbol: String::from(ship_symbol),
            trait_name: String::from(MARKET_TRAIT),
            system: ship_response.nav.system_symbol,
        }
        .into())
    }
}
