use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostCredits {
    #[serde(rename = "agentSymbol")]
    agent_symbol: String,
    credits: i128,
}
