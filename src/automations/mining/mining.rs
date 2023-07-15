use super::find_trade_good;
use super::find_trait;
use super::mine;
use super::refuel_ship;
use super::sell_cargo;
use super::State;
use super::{look_for_fuel, look_for_market, look_for_mine};
use super::{navigate_to_fuel, navigate_to_market, navigate_to_mine};
use super::{FUEL_SYMBOL, MARKET_TRAIT, MINING_WAYPOINT_TYPE};
use crate::queries::Query;
use crate::{local_data::Credentials, queries};
use log::{info, trace};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn mining(
    client: Client,
    sender: Sender<Query>,
    credentials: Credentials,
    ship_symbol: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Started mining task for ship {ship_symbol}");

    let mut state = determine_state(&client, &sender, &credentials.token, &ship_symbol).await?;

    loop {
        trace!("Running mining task for ship {ship_symbol} with state {state}");
        match state {
            State::Mining => {
                state = mine(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            State::LookingForMarket => {
                state = look_for_market(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            State::NavigatingToMarket { arrival } => {
                state =
                    navigate_to_market(&client, &sender, &credentials.token, &ship_symbol, arrival)
                        .await?;
            }
            State::Selling => {
                state = sell_cargo(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            // Need to add a fulfill contract step
            State::LookingForFuel => {
                state = look_for_fuel(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            State::NavigatingToFuel { arrival } => {
                state = navigate_to_fuel(arrival).await?;
            }
            State::Refuelling => {
                state = refuel_ship(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            State::LookingForMine => {
                state = look_for_mine(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            State::NavigatingToMine { arrival } => {
                state =
                    navigate_to_mine(&client, &sender, &credentials.token, &ship_symbol, arrival)
                        .await?;
            }
            State::OutOfFuel => {
                state = State::LookingForFuel;
            }
        }
    }
}

async fn determine_state(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let ship_response = queries::ship(client, sender, token, ship_symbol).await?;

    let cargo_capacity = ship_response.cargo.capacity;
    let cargo_units = ship_response.cargo.units;
    let fuel_capacity = ship_response.fuel.capacity;
    let fuel_units = ship_response.fuel.current;
    let status = ship_response.nav.status;
    let system = ship_response.nav.system_symbol;
    let waypoint = ship_response.nav.waypoint_symbol;
    let waypoint_response = queries::waypoint(client, sender, token, &system, &waypoint).await?;

    let marketplace_trait = find_trait(&waypoint_response.traits, MARKET_TRAIT);

    if fuel_units == 0 {
        Ok(State::OutOfFuel)
    } else if cargo_units == cargo_capacity {
        match status.as_str() {
            "IN_TRANSIT" => Ok(State::NavigatingToMarket {
                arrival: ship_response.nav.route.arrival,
            }),
            _ => match marketplace_trait {
                Some(_) => Ok(State::Selling),
                None => Ok(State::LookingForMarket),
            },
        }
    } else if status == "IN_TRANSIT" {
        let destination_type = ship_response.nav.route.destination.location_type;
        match destination_type.as_str() {
            MINING_WAYPOINT_TYPE => Ok(State::NavigatingToMine {
                arrival: ship_response.nav.route.arrival,
            }),
            _ => Ok(State::NavigatingToFuel {
                arrival: ship_response.nav.route.arrival,
            }),
        }
    } else {
        match waypoint_response.waypoint_type.as_str() {
            MINING_WAYPOINT_TYPE => Ok(State::Mining),
            _ => {
                if fuel_units == fuel_capacity {
                    Ok(State::LookingForMine)
                } else {
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

                            let fuel_trade_good =
                                find_trade_good(&marketplace_response.trade_goods, FUEL_SYMBOL);

                            match fuel_trade_good {
                                Some(_) => Ok(State::Refuelling),
                                None => Ok(State::LookingForFuel),
                            }
                        }
                        None => Ok(State::LookingForFuel),
                    }
                }
            }
        }
    }
}
