use super::FuelConsumedData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FuelData {
    current: u128,
    capacity: u128,
    consumed: FuelConsumedData,
}
