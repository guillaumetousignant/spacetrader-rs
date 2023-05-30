use super::TraitData;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionData {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: Waypoint,
    pub traits: Vec<TraitData>,
    #[serde(rename = "isRecruiting")]
    pub is_recruiting: bool,
}
