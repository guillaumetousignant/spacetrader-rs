use super::ShipData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ship {
    pub data: ShipData,
}
