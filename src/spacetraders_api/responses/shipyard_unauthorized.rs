use super::ShipType;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardUnauthorized {
    pub symbol: Waypoint,
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<ShipType>,
}
