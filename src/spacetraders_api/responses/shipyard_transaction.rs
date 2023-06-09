use crate::spacetraders_api::Waypoint;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardTransaction {
    #[serde(rename = "shipSymbol")]
    ship_symbol: String,
    #[serde(rename = "waypointSymbol")]
    waypoint_symbol: Waypoint,
    #[serde(rename = "agentSymbol")]
    agent_symbol: String,
    price: u128,
    timestamp: DateTime<Utc>,
}
