use super::ChartData;
use super::FactionSymbolData;
use super::OrbitalData;
use super::TraitData;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointData {
    #[serde(rename = "systemSymbol")]
    system_symbol: System,
    symbol: Waypoint,
    #[serde(rename = "type")]
    waypoint_type: String, // Make enum?
    x: i128,
    y: i128,
    orbitals: Vec<OrbitalData>,
    traits: Vec<TraitData>,
    chart: ChartData,
    faction: FactionSymbolData,
}
