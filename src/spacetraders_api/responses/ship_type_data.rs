use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipTypeData {
    #[serde(rename = "type")]
    ship_type: String,
}
