use super::FactionSymbolData;
use super::WaypointLocationData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemData {
    symbol: String,
    #[serde(rename = "sectorSymbol")]
    sector_symbol: String,
    #[serde(rename = "type")]
    system_type: String, // Good candidate for enum
    x: i128,
    y: i128,
    waypoints: Vec<WaypointLocationData>,
    factions: Vec<FactionSymbolData>,
}
