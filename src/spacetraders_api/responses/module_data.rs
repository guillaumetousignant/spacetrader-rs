use super::RequirementsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleData {
    symbol: String,
    name: String,
    description: String,
    capacity: Option<i128>,
    requirements: RequirementsData,
}
