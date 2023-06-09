use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardEngine {
    symbol: String,
    name: String,
    description: String,
    speed: i128,
    requirements: Requirements,
}
