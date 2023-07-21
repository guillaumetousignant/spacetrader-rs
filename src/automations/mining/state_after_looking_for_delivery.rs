use super::State;
use crate::spacetraders_api::responses::{Contract, Delivery};
use chrono::{DateTime, Utc};

pub fn state_after_looking_for_delivery(
    arrival: Option<DateTime<Utc>>,
    contract: Contract,
    delivery: Delivery,
    units: u128,
) -> State {
    match arrival {
        Some(arrival) => State::NavigatingToDelivery {
            arrival,
            contract,
            delivery,
            units,
        },
        None => State::Delivering {
            contract,
            delivery,
            units,
        },
    }
}
