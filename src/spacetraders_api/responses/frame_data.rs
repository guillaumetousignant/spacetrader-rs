use super::RequirementsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameData {
    symbol: String,
    name: String,
    description: String,
    #[serde(rename = "moduleSlots")]
    module_slots: u128,
    #[serde(rename = "mountingPoints")]
    mounting_points: u128,
    #[serde(rename = "fuelCapacity")]
    fuel_capacity: u128,
    condition: i128,
    requirements: RequirementsData,
}
