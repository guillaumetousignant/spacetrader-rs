use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    #[serde(rename = "type")]
    pub rate_limit_type: String,
    #[serde(rename = "retryAfter")]
    pub retry_after: f64,
    #[serde(rename = "limitBurst")]
    pub limit_burst: i128,
    #[serde(rename = "limitPerSecond")]
    pub limit_per_second: i128,
    pub remaining: i128,
    pub reset: DateTime<Utc>,
}
