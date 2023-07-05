use super::Chart;
use super::FactionSymbol;
use super::Meta;
use super::Orbital;
use super::Trait;
use crate::spacetraders_api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    #[serde(rename = "systemSymbol")]
    pub system_symbol: spacetraders_api::System,
    pub symbol: spacetraders_api::Waypoint,
    #[serde(rename = "type")]
    pub waypoint_type: String, // Make enum?
    pub x: i128,
    pub y: i128,
    pub orbitals: Vec<Orbital>,
    pub traits: Vec<Trait>,
    pub chart: Chart,
    pub faction: FactionSymbol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointResponse {
    pub data: Waypoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoints {
    pub data: Vec<Waypoint>,
    pub meta: Meta,
}
