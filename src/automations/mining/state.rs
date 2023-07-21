use crate::spacetraders_api::responses::{Contract, Delivery};
use chrono::{DateTime, Utc};
use std::fmt;

pub enum State {
    Mining,
    LookingForMarket,
    NavigatingToMarket {
        arrival: DateTime<Utc>,
    },
    Selling,
    LookingForDelivery {
        contract: Contract,
        delivery: Delivery,
        units: u128,
    },
    NavigatingToDelivery {
        arrival: DateTime<Utc>,
        contract: Contract,
        delivery: Delivery,
        units: u128,
    },
    Delivering {
        contract: Contract,
        delivery: Delivery,
        units: u128,
    },
    LookingForFuel,
    NavigatingToFuel {
        arrival: DateTime<Utc>,
    },
    Refuelling,
    LookingForMine,
    NavigatingToMine {
        arrival: DateTime<Utc>,
    },
    OutOfFuel,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Mining => write!(f, "Mining"),
            State::LookingForMarket => write!(f, "LookingForMarket"),
            State::NavigatingToMarket { arrival } => {
                write!(f, "NavigatingToMarket, arriving at {arrival}")
            }
            State::Selling => write!(f, "Selling"),
            State::LookingForDelivery {
                contract,
                delivery,
                units,
            } => {
                write!(
                    f,
                    "LookingForDelivery, contract {} to deliver {} units of  {} to {}",
                    contract.id, units, delivery.trade_symbol, delivery.destination_symbol
                )
            }
            State::NavigatingToDelivery {
                arrival,
                contract,
                delivery,
                units,
            } => {
                write!(
                    f,
                    "NavigatingToDelivery, arriving at {arrival} for contract {} to deliver {} units of {} to {}",
                    contract.id, units, delivery.trade_symbol, delivery.destination_symbol
                )
            }
            State::Delivering {
                contract,
                delivery,
                units,
            } => {
                write!(
                    f,
                    "Delivering, contract {} to deliver {} units of {} to {}",
                    contract.id, units, delivery.trade_symbol, delivery.destination_symbol
                )
            }
            State::LookingForFuel => write!(f, "LookingForFuel"),
            State::NavigatingToFuel { arrival } => {
                write!(f, "NavigatingToFuel, arriving at {arrival}")
            }
            State::Refuelling => write!(f, "Refuelling"),
            State::LookingForMine => write!(f, "LookingForMine"),
            State::NavigatingToMine { arrival } => {
                write!(f, "NavigatingToMine, arriving at {arrival}")
            }
            State::OutOfFuel => write!(f, "OutOfFuel"),
        }
    }
}
