use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    symbol: Waypoint,
    #[serde(rename = "type")]
    location_type: String, // Make enum?
    #[serde(rename = "systemSymbol")]
    system_symbol: System,
    x: i128,
    y: i128,
}