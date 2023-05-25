use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentData {
    #[serde(rename = "onAccepted")]
    on_accepted: u128,
    #[serde(rename = "onFulfilled")]
    on_fulfilled: u128,
}
