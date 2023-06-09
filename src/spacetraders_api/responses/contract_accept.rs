use super::Agent;
use super::Contract;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAccept {
    contract: Contract,
    agent: Agent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractAcceptResponse {
    pub data: ContractAccept,
}
