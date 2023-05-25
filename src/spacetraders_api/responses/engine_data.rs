use super::RequirementsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineData {
    symbol: String,
    name: String,
    description: String,
    condition: i128,
    speed: i128,
    requirements: RequirementsData,
}
