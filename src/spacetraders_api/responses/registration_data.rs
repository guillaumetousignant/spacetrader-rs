use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationData {
    name: String,
    #[serde(rename = "factionSymbol")]
    faction_symbol: String,
    role: String,
}
