use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "onAccepted")]
    on_accepted: u128,
    #[serde(rename = "onFulfilled")]
    on_fulfilled: u128,
}
