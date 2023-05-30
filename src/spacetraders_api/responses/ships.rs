use super::MetaData;
use super::ShipData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ships {
    pub data: Vec<ShipData>,
    pub meta: MetaData,
}
