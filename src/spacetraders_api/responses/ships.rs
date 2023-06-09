use super::MetaData;
use super::Ship;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ships {
    pub data: Vec<Ship>,
    pub meta: MetaData,
}
