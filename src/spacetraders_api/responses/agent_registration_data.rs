use super::AgentData;
use super::ContractData;
use super::FactionData;
use super::ShipData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRegistrationData {
    pub token: String,
    agent: AgentData,
    contract: ContractData,
    faction: FactionData,
    ship: ShipData,
}
