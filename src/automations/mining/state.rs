use chrono::{DateTime, Utc};
use std::fmt;

pub enum State {
    Mining,
    LookingForMarket,
    NavigatingToMarket { arrival: DateTime<Utc> },
    Selling,
    LookingForFuel,
    NavigatingToFuel { arrival: DateTime<Utc> },
    Refuelling,
    LookingForMine,
    NavigatingToMine { arrival: DateTime<Utc> },
    OutOfFuel,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            State::Mining => write!(f, "Mining"),
            State::LookingForMarket => write!(f, "LookingForMarket"),
            State::NavigatingToMarket { arrival } => {
                write!(f, "NavigatingToMarket, arriving at {arrival}")
            }
            State::Selling => write!(f, "Selling"),
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
