use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    pub total: u128,
    pub page: u128,
    pub limit: u128,
}
