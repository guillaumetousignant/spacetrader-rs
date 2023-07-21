use super::Route;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nav {
    #[serde(rename = "systemSymbol")]
    pub system_symbol: System,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: Waypoint,
    pub route: Route,
    pub status: String,
    #[serde(rename = "flightMode")]
    pub flight_mode: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavResponse {
    pub data: Nav,
}
