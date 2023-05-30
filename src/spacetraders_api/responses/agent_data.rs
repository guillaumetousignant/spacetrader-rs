use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentData {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: Waypoint,
    pub credits: i128,
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
}
