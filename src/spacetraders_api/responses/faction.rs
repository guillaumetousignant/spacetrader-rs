use super::Trait;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: Waypoint,
    pub traits: Vec<Trait>,
    #[serde(rename = "isRecruiting")]
    pub is_recruiting: bool,
}
