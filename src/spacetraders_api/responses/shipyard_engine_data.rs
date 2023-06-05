use super::RequirementsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipyardEngineData {
    symbol: String,
    name: String,
    description: String,
    speed: i128,
    requirements: RequirementsData,
}
