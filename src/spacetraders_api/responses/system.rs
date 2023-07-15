use super::FactionSymbol;
use super::Meta;
use super::WaypointLocation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    pub symbol: String,
    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,
    #[serde(rename = "type")]
    pub system_type: String, // Good candidate for enum
    pub x: i128,
    pub y: i128,
    pub waypoints: Vec<WaypointLocation>,
    pub factions: Vec<FactionSymbol>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemResponse {
    pub data: System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Systems {
    pub data: Vec<System>,
    pub meta: Meta,
}
