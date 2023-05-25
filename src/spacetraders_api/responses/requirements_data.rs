use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequirementsData {
    power: Option<i128>,
    crew: Option<u128>,
    slots: Option<u128>,
}
