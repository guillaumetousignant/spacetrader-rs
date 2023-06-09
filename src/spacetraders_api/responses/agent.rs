use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: Waypoint,
    pub credits: i128,
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub data: Agent,
}
