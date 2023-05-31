use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub page: Option<u128>,
    pub limit: Option<u128>,
}
