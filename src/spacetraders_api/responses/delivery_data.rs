use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryData {
    #[serde(rename = "tradeSymbol")]
    trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    destination_symbol: Waypoint,
    #[serde(rename = "unitsRequired")]
    units_required: u128,
    #[serde(rename = "unitsFulfilled")]
    units_fulfilled: u128,
}
