use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    symbol: String,
    name: String,
    description: String,
    units: u128,
}
