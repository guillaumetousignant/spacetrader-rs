use super::MostCreditsData;
use super::MostSubmittedChartsData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeaderboardData {
    #[serde(rename = "mostCredits")]
    most_credits: Vec<MostCreditsData>,
    #[serde(rename = "mostSubmittedCharts")]
    most_submitted_charts: Vec<MostSubmittedChartsData>,
}
