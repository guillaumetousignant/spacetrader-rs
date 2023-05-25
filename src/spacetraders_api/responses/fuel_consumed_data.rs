use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelConsumedData {
    amount: u128,
    timestamp: DateTime<Utc>,
}
