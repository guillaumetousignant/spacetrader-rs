use super::Yield;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extraction {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "yield")]
    pub yield_data: Yield, // yield is a keyword
}
