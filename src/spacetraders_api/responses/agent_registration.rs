use super::AgentRegistrationData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRegistration {
    pub data: AgentRegistrationData,
}
