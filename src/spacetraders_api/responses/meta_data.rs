use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    pub total: u128,
    pub page: u128,
    pub limit: u128,
}
