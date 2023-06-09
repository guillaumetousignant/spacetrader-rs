use super::Agent;
use super::Contract;
use super::Faction;
use super::Ship;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistration {
    pub token: String,
    pub agent: Agent,
    pub contract: Contract,
    pub faction: Faction,
    pub ship: Ship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistrationResponse {
    pub data: AgentRegistration,
}
