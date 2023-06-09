use super::Requirements;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    symbol: String,
    name: String,
    description: String,
    capacity: Option<i128>,
    requirements: Requirements,
}
