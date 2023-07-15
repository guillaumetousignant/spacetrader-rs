use super::InventoryItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cargo {
    pub capacity: u128,
    pub units: u128,
    pub inventory: Vec<InventoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CargoResponse {
    pub data: Cargo,
}
