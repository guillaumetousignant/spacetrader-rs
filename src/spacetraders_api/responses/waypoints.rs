use super::MetaData;
use super::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Waypoints {
    pub data: Vec<Waypoint>,
    pub meta: MetaData,
}
