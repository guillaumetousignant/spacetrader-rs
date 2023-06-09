use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Crew {
    current: u128,
    capacity: u128,
    required: u128,
    rotation: String,
    morale: u128,
    wages: u128,
}
