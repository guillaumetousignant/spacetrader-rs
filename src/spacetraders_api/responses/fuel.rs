use super::FuelConsumed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fuel {
    pub current: u128,
    pub capacity: u128,
    pub consumed: FuelConsumed,
}
