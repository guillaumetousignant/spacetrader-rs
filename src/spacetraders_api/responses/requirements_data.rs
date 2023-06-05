use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequirementsData {
    power: Option<i128>,
    crew: Option<i128>,
    slots: Option<i128>,
}
