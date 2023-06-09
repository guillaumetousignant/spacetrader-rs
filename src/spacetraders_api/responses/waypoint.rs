use super::Chart;
use super::FactionSymbol;
use super::Orbital;
use super::Trait;
use crate::spacetraders_api;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoint {
    #[serde(rename = "systemSymbol")]
    system_symbol: spacetraders_api::System,
    symbol: spacetraders_api::Waypoint,
    #[serde(rename = "type")]
    waypoint_type: String, // Make enum?
    x: i128,
    y: i128,
    orbitals: Vec<Orbital>,
    traits: Vec<Trait>,
    chart: Chart,
    faction: FactionSymbol,
}
