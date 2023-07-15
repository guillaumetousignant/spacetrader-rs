use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navigate {
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: Waypoint,
}
