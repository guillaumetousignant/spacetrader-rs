use crate::spacetraders_api::Waypoint;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketTransaction {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: Waypoint,
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "type")]
    pub transaction_type: String,
    pub units: u128,
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i128,
    #[serde(rename = "totalPrice")]
    pub total_price: i128,
    pub timestamp: DateTime<Utc>,
}
