use super::DeliveryData;
use super::PaymentData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TermsData {
    deadline: DateTime<Utc>,
    payment: PaymentData,
    deliver: Vec<DeliveryData>,
}
