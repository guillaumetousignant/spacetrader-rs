use super::Agent;
use super::Fuel;
use super::MarketTransaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refuel {
    pub agent: Agent,
    pub fuel: Fuel,
    pub transaction: MarketTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefuelResponse {
    pub data: Refuel,
}
