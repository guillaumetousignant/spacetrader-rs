use super::WaypointData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Waypoint {
    data: WaypointData,
}
