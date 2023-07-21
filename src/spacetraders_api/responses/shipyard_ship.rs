use super::Module;
use super::Mount;
use super::ShipyardEngine;
use super::ShipyardFrame;
use super::ShipyardReactorData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    pub ship_type: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "purchasePrice")]
    pub purchase_price: u128,
    pub frame: ShipyardFrame,
    pub reactor: ShipyardReactorData,
    pub engine: ShipyardEngine,
    pub modules: Vec<Module>,
    pub mounts: Vec<Mount>,
}
