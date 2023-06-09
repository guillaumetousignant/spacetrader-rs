use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostSubmittedCharts {
    #[serde(rename = "agentSymbol")]
    agent_symbol: String,
    #[serde(rename = "chartCount")]
    chart_count: i128,
}
