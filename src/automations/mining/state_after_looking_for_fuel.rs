use super::State;
use chrono::{DateTime, Utc};

pub fn state_after_looking_for_fuel(arrival: Option<DateTime<Utc>>) -> State {
    match arrival {
        Some(arrival) => State::NavigatingToFuel { arrival },
        None => State::Refuelling,
    }
}
