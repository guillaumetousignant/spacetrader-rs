use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mount {
    symbol: String,
    name: String,
    description: String,
    strength: Option<i128>,
    deposits: Option<Vec<String>>,
    requirements: Requirements,
}
