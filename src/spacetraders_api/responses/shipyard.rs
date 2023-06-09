use super::ShipType;
use super::ShipyardShip;
use super::ShipyardTransaction;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipyard {
    pub symbol: Waypoint,
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<ShipType>,
    pub transactions: Vec<ShipyardTransaction>,
    pub ships: Vec<ShipyardShip>,
}
