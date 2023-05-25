use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryItemData {
    symbol: String,
    name: String,
    description: String,
    units: u128,
}
