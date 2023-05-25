use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CrewData {
    current: u128,
    capacity: u128,
    required: u128,
    rotation: String,
    morale: u128,
    wages: u128,
}
