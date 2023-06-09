use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipType {
    #[serde(rename = "type")]
    ship_type: String,
}
