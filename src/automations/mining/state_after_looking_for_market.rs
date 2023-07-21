use super::State;
use chrono::{DateTime, Utc};

pub fn state_after_looking_for_market(arrival: Option<DateTime<Utc>>) -> State {
    match arrival {
        Some(arrival) => State::NavigatingToMarket { arrival },
        None => State::Selling,
    }
}
