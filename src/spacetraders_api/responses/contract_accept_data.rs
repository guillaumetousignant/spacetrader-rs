use super::AgentData;
use super::ContractData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractAcceptData {
    contract: ContractData,
    agent: AgentData,
}
