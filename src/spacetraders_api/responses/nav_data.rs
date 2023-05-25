use super::RouteData;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NavData {
    #[serde(rename = "systemSymbol")]
    system_symbol: System,
    #[serde(rename = "waypointSymbol")]
    waypoint_symbol: Waypoint,
    route: RouteData,
    status: String,
    #[serde(rename = "flightMode")]
    flight_mode: String,
}
