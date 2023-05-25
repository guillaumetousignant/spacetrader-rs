use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentData {
    #[serde(rename = "accountId")]
    account_id: String,
    symbol: String,
    headquarters: Waypoint,
    credits: i128,
    #[serde(rename = "startingFaction")]
    starting_faction: String,
}
