use super::FactionSymbol;
use super::Meta;
use super::WaypointLocation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct System {
    symbol: String,
    #[serde(rename = "sectorSymbol")]
    sector_symbol: String,
    #[serde(rename = "type")]
    system_type: String, // Good candidate for enum
    x: i128,
    y: i128,
    waypoints: Vec<WaypointLocation>,
    factions: Vec<FactionSymbol>,
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
