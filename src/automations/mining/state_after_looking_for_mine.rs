use super::State;
use chrono::{DateTime, Utc};

pub fn state_after_looking_for_mine(arrival: Option<DateTime<Utc>>) -> State {
    match arrival {
        Some(arrival) => State::NavigatingToMine { arrival },
        None => State::Mining,
    }
}
