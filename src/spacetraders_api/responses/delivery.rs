use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delivery {
    #[serde(rename = "tradeSymbol")]
    trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    destination_symbol: Waypoint,
    #[serde(rename = "unitsRequired")]
    units_required: u128,
    #[serde(rename = "unitsFulfilled")]
    units_fulfilled: u128,
}
