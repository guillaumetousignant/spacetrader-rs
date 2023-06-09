use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
    symbol: String,
    name: String,
    description: String,
    condition: i128,
    speed: i128,
    requirements: Requirements,
}
