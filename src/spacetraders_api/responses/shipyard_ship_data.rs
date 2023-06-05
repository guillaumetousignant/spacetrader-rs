use super::ModuleData;
use super::MountData;
use super::ShipyardEngineData;
use super::ShipyardFrameData;
use super::ShipyardReactorData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipyardShipData {
    #[serde(rename = "type")]
    ship_type: String,
    name: String,
    description: String,
    #[serde(rename = "purchasePrice")]
    purchase_price: u128,
    frame: ShipyardFrameData,
    reactor: ShipyardReactorData,
    engine: ShipyardEngineData,
    modules: Vec<ModuleData>,
    mounts: Vec<MountData>,
}
