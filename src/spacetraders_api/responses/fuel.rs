use super::FuelConsumed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fuel {
    current: u128,
    capacity: u128,
    consumed: FuelConsumed,
}
