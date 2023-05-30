use super::MetaData;
use super::WaypointData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Waypoints {
    pub data: Vec<WaypointData>,
    pub meta: MetaData,
}
