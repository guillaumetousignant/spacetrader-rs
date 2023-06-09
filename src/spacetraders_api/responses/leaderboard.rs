use super::MostCredits;
use super::MostSubmittedCharts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    #[serde(rename = "mostCredits")]
    most_credits: Vec<MostCredits>,
    #[serde(rename = "mostSubmittedCharts")]
    most_submitted_charts: Vec<MostSubmittedCharts>,
}
