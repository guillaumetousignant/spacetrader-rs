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
