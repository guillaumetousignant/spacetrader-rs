use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooldown {
    #[serde(rename = "shipSymbol")]
    pub ship_symbol: String,
    #[serde(rename = "totalSeconds")]
    pub total_seconds: i128,
    #[serde(rename = "remainingSeconds")]
    pub remaining_seconds: i128,
    pub expiration: DateTime<Utc>,
}
