use super::Route;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nav {
    #[serde(rename = "systemSymbol")]
    system_symbol: System,
    #[serde(rename = "waypointSymbol")]
    waypoint_symbol: Waypoint,
    route: Route,
    status: String,
    #[serde(rename = "flightMode")]
    flight_mode: String,
}
