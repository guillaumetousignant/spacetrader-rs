use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MostSubmittedChartsData {
    #[serde(rename = "agentSymbol")]
    agent_symbol: String,
    #[serde(rename = "chartCount")]
    chart_count: i128,
}
