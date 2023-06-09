use super::Faction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactionResponse {
    pub data: Faction,
}
