use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactor {
    symbol: String,
    name: String,
    description: String,
    condition: i128,
    #[serde(rename = "powerOutput")]
    power_output: i128,
    requirements: Requirements,
}
