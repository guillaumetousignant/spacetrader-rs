use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
    #[serde(rename = "submittedBy")]
    submitted_by: String,
    #[serde(rename = "submittedOn")]
    submitted_on: DateTime<Utc>,
}
