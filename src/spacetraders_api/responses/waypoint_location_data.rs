use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WaypointLocationData {
    symbol: Waypoint,
    #[serde(rename = "type")]
    location_type: String, // Make enum?
    x: i128,
    y: i128,
}
