use super::Ship;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipResponse {
    pub data: Ship,
}
