use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliver {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    pub units: u128,
}
