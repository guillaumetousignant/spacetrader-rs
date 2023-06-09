use super::Module;
use super::Mount;
use super::ShipyardEngine;
use super::ShipyardFrame;
use super::ShipyardReactorData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardShip {
    #[serde(rename = "type")]
    ship_type: String,
    name: String,
    description: String,
    #[serde(rename = "purchasePrice")]
    purchase_price: u128,
    frame: ShipyardFrame,
    reactor: ShipyardReactorData,
    engine: ShipyardEngine,
    modules: Vec<Module>,
    mounts: Vec<Mount>,
}
