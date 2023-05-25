use super::RequirementsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MountData {
    symbol: String,
    name: String,
    description: String,
    strength: i128,
    deposits: Option<Vec<String>>,
    requirements: RequirementsData,
}
