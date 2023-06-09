use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub symbol: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "moduleSlots")]
    pub module_slots: u128,
    #[serde(rename = "mountingPoints")]
    pub mounting_points: u128,
    #[serde(rename = "fuelCapacity")]
    pub fuel_capacity: u128,
    pub condition: i128,
    pub requirements: Requirements,
}
