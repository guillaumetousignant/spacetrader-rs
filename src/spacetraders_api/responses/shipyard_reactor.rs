use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardReactorData {
    symbol: String,
    name: String,
    description: String,
    #[serde(rename = "powerOutput")]
    power_output: i128,
    requirements: Requirements,
}
