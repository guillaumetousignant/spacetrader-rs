use super::RequirementsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ReactorData {
    symbol: String,
    name: String,
    description: String,
    condition: i128,
    #[serde(rename = "powerOutput")]
    power_output: i128,
    requirements: RequirementsData,
}
