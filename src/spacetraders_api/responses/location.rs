use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub symbol: Waypoint,
    #[serde(rename = "type")]
    pub location_type: String, // Make enum?
    #[serde(rename = "systemSymbol")]
    pub system_symbol: System,
    pub x: i128,
    pub y: i128,
}
