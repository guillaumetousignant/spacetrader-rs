use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub name: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    pub role: String,
}
