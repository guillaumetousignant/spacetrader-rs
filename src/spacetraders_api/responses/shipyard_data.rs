use super::ShipTypeData;
use super::ShipyardShipData;
use super::ShipyardTransactionData;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipyardData {
    pub symbol: Waypoint,
    #[serde(rename = "shipTypes")]
    pub ship_types: Vec<ShipTypeData>,
    pub transactions: Vec<ShipyardTransactionData>,
    pub ships: Vec<ShipyardShipData>,
}
