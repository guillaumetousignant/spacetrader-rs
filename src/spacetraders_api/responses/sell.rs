use super::Agent;
use super::Cargo;
use super::MarketTransaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sell {
    pub agent: Agent,
    pub cargo: Cargo,
    pub transaction: MarketTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellResponse {
    pub data: Sell,
}
