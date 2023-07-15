use super::Delivery;
use super::Payment;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Terms {
    pub deadline: DateTime<Utc>,
    pub payment: Payment,
    pub deliver: Vec<Delivery>,
}
