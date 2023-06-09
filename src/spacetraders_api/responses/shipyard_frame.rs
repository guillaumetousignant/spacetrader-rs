use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardFrame {
    symbol: String,
    name: String,
    description: String,
    #[serde(rename = "moduleSlots")]
    module_slots: u128,
    #[serde(rename = "mountingPoints")]
    mounting_points: u128,
    #[serde(rename = "fuelCapacity")]
    fuel_capacity: u128,
    requirements: Requirements,
}
