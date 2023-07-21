use super::Agent;
use super::Contract;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fulfill {
    pub agent: Agent,
    pub contract: Contract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillResponse {
    pub data: Fulfill,
}
