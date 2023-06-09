use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuelConsumed {
    amount: u128,
    timestamp: DateTime<Utc>,
}
