use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    power: Option<i128>,
    crew: Option<i128>,
    slots: Option<i128>,
}
