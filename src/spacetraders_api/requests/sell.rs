use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sell {
    pub symbol: String,
    pub units: u128,
}
