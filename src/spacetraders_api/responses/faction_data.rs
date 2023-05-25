use super::TraitData;
use crate::spacetraders_api::Waypoint;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FactionData {
    symbol: String,
    name: String,
    description: String,
    headquarters: Waypoint,
    traits: Vec<TraitData>,
    #[serde(rename = "isRecruiting")]
    is_recruiting: bool,
}
