use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartData {
    #[serde(rename = "submittedBy")]
    submitted_by: String,
    #[serde(rename = "submittedOn")]
    submitted_on: DateTime<Utc>,
}
