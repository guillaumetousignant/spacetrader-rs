use super::InventoryItemData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CargoData {
    capacity: u128,
    units: u128,
    inventory: Vec<InventoryItemData>,
}
