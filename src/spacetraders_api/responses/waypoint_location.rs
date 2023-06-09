use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaypointLocation {
    symbol: Waypoint,
    #[serde(rename = "type")]
    location_type: String, // Make enum?
    x: i128,
    y: i128,
}
