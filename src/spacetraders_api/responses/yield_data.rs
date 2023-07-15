use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yield {
    pub symbol: String,
    pub units: u128,
}
