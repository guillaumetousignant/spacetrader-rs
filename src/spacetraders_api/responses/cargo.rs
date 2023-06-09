use super::InventoryItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cargo {
    capacity: u128,
    units: u128,
    inventory: Vec<InventoryItem>,
}
