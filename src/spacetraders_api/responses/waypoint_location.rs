use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointLocation {
    pub symbol: Waypoint,
    #[serde(rename = "type")]
    pub waypoint_type: String, // Make enum?
    pub x: i128,
    pub y: i128,
}
