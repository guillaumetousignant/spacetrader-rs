use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delivery {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    pub destination_symbol: Waypoint,
    #[serde(rename = "unitsRequired")]
    pub units_required: u128,
    #[serde(rename = "unitsFulfilled")]
    pub units_fulfilled: u128,
}

impl PartialEq for Delivery {
    fn eq(&self, other: &Self) -> bool {
        self.trade_symbol == other.trade_symbol
            && self.destination_symbol == other.destination_symbol
            && self.units_required == other.units_required
            && self.units_fulfilled == other.units_fulfilled
    }
}
