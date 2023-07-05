use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipType {
    #[serde(rename = "type")]
    pub ship_type: String,
}
