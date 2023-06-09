use super::Delivery;
use super::Payment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    deadline: DateTime<Utc>,
    payment: Payment,
    deliver: Vec<Delivery>,
}
