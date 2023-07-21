use super::State;
use super::{
    state_after_delivering, state_after_looking_for_delivery, state_after_looking_for_fuel,
    state_after_looking_for_market, state_after_looking_for_mine, state_after_mining,
    state_after_selling,
};
use crate::automations::actions::deliver;
use crate::automations::actions::mine;
use crate::automations::actions::refuel_ship;
use crate::automations::actions::sell_cargo;
use crate::automations::actions::try_refuel;
use crate::automations::actions::{
    look_for_delivery, look_for_fuel, look_for_market, look_for_mine,
};
use crate::automations::utilities::wait_until;
use crate::automations::utilities::{find_delivery_in_contracts, find_trade_good, find_trait};
use crate::automations::utilities::{FUEL_SYMBOL, MARKET_TRAIT, MINING_WAYPOINT_TYPE};
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
                let cargo = mine(&client, &sender, &credentials.token, &ship_symbol).await?;
                let refuelled =
                    try_refuel(&client, &sender, &credentials.token, &ship_symbol).await?;

                state = state_after_mining(
                    &client,
                    &sender,
                    &credentials.token,
                    &ship_symbol,
                    cargo,
                    refuelled,
                )
                .await?;
            }
            State::LookingForMarket => {
                let arrival =
                    look_for_market(&client, &sender, &credentials.token, &ship_symbol).await?;
                state = state_after_looking_for_market(arrival);
            }
            State::NavigatingToMarket { arrival } => {
                info!("Ship {ship_symbol} navigating to market, arriving at {arrival}");
                wait_until(arrival).await?;
                state = State::Selling;
            }
            State::Selling => {
                let cargo = sell_cargo(&client, &sender, &credentials.token, &ship_symbol).await?;
                let refuelled =
                    try_refuel(&client, &sender, &credentials.token, &ship_symbol).await?;
                state = state_after_selling(&client, &sender, &credentials.token, cargo, refuelled)
                    .await?;
            }
            State::LookingForDelivery {
                contract,
                delivery,
                units,
            } => {
                let arrival = look_for_delivery(
                    &client,
                    &sender,
                    &credentials.token,
                    &ship_symbol,
                    &delivery,
                )
                .await?;
                state = state_after_looking_for_delivery(arrival, contract, delivery, units);
            }
            State::NavigatingToDelivery {
                arrival,
                contract,
                delivery,
                units,
            } => {
                info!("Ship {ship_symbol} navigating to delivery, arriving at {arrival}");
                wait_until(arrival).await?;
                state = State::Delivering {
                    contract,
                    delivery,
                    units,
                };
            }
            State::Delivering {
                contract,
                delivery,
                units,
            } => {
                let cargo = deliver(
                    &client,
                    &sender,
                    &credentials.token,
                    &ship_symbol,
                    contract,
                    delivery,
                    units,
                )
                .await?;

                let refuelled =
                    try_refuel(&client, &sender, &credentials.token, &ship_symbol).await?;
                state = state_after_delivering(
                    &client,
                    &sender,
                    &credentials.token,
                    &ship_symbol,
                    cargo,
                    refuelled,
                )
                .await?;
            }
            State::LookingForFuel => {
                let arrival =
                    look_for_fuel(&client, &sender, &credentials.token, &ship_symbol).await?;
                state = state_after_looking_for_fuel(arrival);
            }
            State::NavigatingToFuel { arrival } => {
                info!("Ship {ship_symbol} navigating to fuel, arriving at {arrival}");
                wait_until(arrival).await?;
                state = State::Refuelling
            }
            State::Refuelling => {
                refuel_ship(&client, &sender, &credentials.token, &ship_symbol).await?;
                state = State::LookingForMine;
            }
            State::LookingForMine => {
                let arrival =
                    look_for_mine(&client, &sender, &credentials.token, &ship_symbol).await?;
                state = state_after_looking_for_mine(arrival);
            }
            State::NavigatingToMine { arrival } => {
                info!("Ship {ship_symbol} navigating to mine, arriving at {arrival}");
                wait_until(arrival).await?;
                state = State::Mining;
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

    // If no fuel, we're out of fuel (duh)
    if fuel_units == 0 {
        return Ok(State::OutOfFuel);
    }

    let waypoint_response = queries::waypoint(client, sender, token, &system, &waypoint).await?;
    let marketplace_trait = find_trait(&waypoint_response.traits, MARKET_TRAIT);
    let contracts_response = queries::contracts(client, sender, token).await?;
    let market_trade_good = ship_response
        .cargo
        .inventory
        .iter()
        .find(|&item| find_delivery_in_contracts(&contracts_response, &item.symbol).is_none());
    let delivery_trade_good = ship_response.cargo.inventory.iter().find_map(|item| {
        find_delivery_in_contracts(&contracts_response, &item.symbol)
            .map(|(contract, delivery)| (contract, delivery, item.units))
    });

    // If the cargo is full, we're either going to the market if there are any non-contract items, or delivering if we have only contract items.
    // We're either in transit, already there, or we havent't left yet
    if cargo_units == cargo_capacity {
        return if status.as_str() == "IN_TRANSIT" {
            if market_trade_good.is_some() {
                Ok(State::NavigatingToMarket {
                    arrival: ship_response.nav.route.arrival,
                })
            } else if let Some((contract, delivery, units)) = delivery_trade_good {
                Ok(State::NavigatingToDelivery {
                    arrival: ship_response.nav.route.arrival,
                    contract: contract.clone(),
                    delivery: delivery.clone(),
                    units,
                })
            }
            // Should not really get here, it means that the cargo is full, but there is no non-contract items and no contract items
            else {
                Ok(State::LookingForFuel)
            }
        } else {
            if market_trade_good.is_some() {
                if marketplace_trait.is_some() {
                    Ok(State::Selling)
                } else {
                    Ok(State::LookingForMarket)
                }
            } else if let Some((contract, delivery, units)) = delivery_trade_good {
                if delivery.destination_symbol == waypoint {
                    Ok(State::Delivering {
                        contract: contract.clone(),
                        delivery: delivery.clone(),
                        units,
                    })
                } else {
                    Ok(State::LookingForDelivery {
                        contract: contract.clone(),
                        delivery: delivery.clone(),
                        units,
                    })
                }
            }
            // Should not really get here, it means that the cargo is full, but there is no non-contract items and no contract items
            else {
                Ok(State::LookingForFuel)
            }
        };
    }

    // If we're in transit on a not-full cargo, either we're going mining, we're going to deliver for a contract, or we're going to refuel
    if status == "IN_TRANSIT" {
        let destination_type = ship_response.nav.route.destination.location_type;
        return if destination_type.as_str() == MINING_WAYPOINT_TYPE {
            Ok(State::NavigatingToMine {
                arrival: ship_response.nav.route.arrival,
            })
        } else if let Some((contract, delivery, units)) = delivery_trade_good {
            Ok(State::NavigatingToDelivery {
                arrival: ship_response.nav.route.arrival,
                contract: contract.clone(),
                delivery: delivery.clone(),
                units,
            })
        } else {
            Ok(State::NavigatingToFuel {
                arrival: ship_response.nav.route.arrival,
            })
        };
    }

    // If we are at a mine, with a non-full cargo, we're mining
    if waypoint_response.waypoint_type.as_str() == MINING_WAYPOINT_TYPE {
        return Ok(State::Mining);
    }

    // If we have delivery items, we're either delivering them, or looking to deliver them
    if let Some((contract, delivery, units)) = delivery_trade_good {
        return if delivery.destination_symbol == waypoint {
            Ok(State::Delivering {
                contract: contract.clone(),
                delivery: delivery.clone(),
                units,
            })
        } else {
            Ok(State::LookingForDelivery {
                contract: contract.clone(),
                delivery: delivery.clone(),
                units,
            })
        };
    };

    // If we are have no delivery items, we're either looking for a mine, refuelling, or looking for fuel
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
