use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaData {
    total: u128,
    page: u128,
    limit: u128,
}
