use super::SystemData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    data: SystemData,
}
