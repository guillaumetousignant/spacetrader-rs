use crate::queries::Query;
use crate::{local_data::Credentials, queries};
use log::{info, trace};
use reqwest::Client;
use std::fmt;
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
                println!("Mining");
                state = State::LookingForMarket;
            }
            State::LookingForMarket => {
                println!("LookingForMarket");
                state = State::NavigatingToMarket;
            }
            State::NavigatingToMarket => {
                println!("NavigatingToMarket");
                state = State::Selling;
            }
            State::Selling => {
                println!("Selling");
                state = State::LookingForFuel;
            }
            State::LookingForFuel => {
                println!("LookingForFuel");
                state = State::NavigatingToFuel;
            }
            State::NavigatingToFuel => {
                println!("NavigatingToFuel");
                state = State::Refuelling;
            }
            State::Refuelling => {
                println!("Refuelling");
                state = State::LookingForMine;
            }
            State::LookingForMine => {
                state = look_for_mine(&client, &sender, &credentials.token, &ship_symbol).await?;
            }
            State::NavigatingToMine => {
                println!("NavigatingToMine");
                state = State::Mining;
            }
            State::OutOfFuel => {
                println!("OutOfFuel");
                state = State::LookingForFuel;
            }
        }
    }
}

enum State {
    Mining,
    LookingForMarket,
    NavigatingToMarket,
    Selling,
    LookingForFuel,
    NavigatingToFuel,
    Refuelling,
    LookingForMine,
    NavigatingToMine,
    OutOfFuel,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Mining => write!(f, "Mining"),
            State::LookingForMarket => write!(f, "LookingForMarket"),
            State::NavigatingToMarket => write!(f, "NavigatingToMarket"),
            State::Selling => write!(f, "Selling"),
            State::LookingForFuel => write!(f, "LookingForFuel"),
            State::NavigatingToFuel => write!(f, "NavigatingToFuel"),
            State::Refuelling => write!(f, "Refuelling"),
            State::LookingForMine => write!(f, "LookingForMine"),
            State::NavigatingToMine => write!(f, "NavigatingToMine"),
            State::OutOfFuel => write!(f, "OutOfFuel"),
        }
    }
}

async fn determine_state(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &String,
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

    let marketplace_trait = waypoint_response
        .traits
        .iter()
        .find(|&trait_iter| trait_iter.symbol == "MARKETPLACE");

    if fuel_units == 0 {
        Ok(State::OutOfFuel)
    } else if cargo_units == cargo_capacity {
        match status.as_str() {
            "IN_TRANSIT" => Ok(State::NavigatingToMarket),
            _ => match marketplace_trait {
                Some(_) => Ok(State::Selling),
                None => Ok(State::LookingForMarket),
            },
        }
    } else if status == "IN_TRANSIT" {
        let destination_type = ship_response.nav.route.destination.location_type;
        match destination_type.as_str() {
            "ASTEROID_FIELD" => Ok(State::NavigatingToMine),
            _ => Ok(State::NavigatingToFuel),
        }
    } else {
        match waypoint_response.waypoint_type.as_str() {
            "ASTEROID_FIELD" => Ok(State::Mining),
            _ => {
                if fuel_units == fuel_capacity {
                    Ok(State::LookingForMine)
                } else {
                    match marketplace_trait {
                        Some(_) => Ok(State::Refuelling),
                        None => Ok(State::LookingForFuel),
                    }
                }
            }
        }
    }
}

async fn look_for_mine(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &String,
) -> Result<State, Box<dyn std::error::Error + Send + Sync>> {
    let _ = queries::orbit(client, sender, token, ship_symbol).await?;
    Ok(State::NavigatingToMine)
}
