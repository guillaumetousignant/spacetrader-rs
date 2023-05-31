use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MostCreditsData {
    #[serde(rename = "agentSymbol")]
    agent_symbol: String,
    credits: i128,
}
