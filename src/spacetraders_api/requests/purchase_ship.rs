use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseShip {
    #[serde(rename = "shipType")]
    pub ship_type: String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: Waypoint,
}
