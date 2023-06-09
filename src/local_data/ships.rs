use super::Ship;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ships {
    pub ships: Vec<Ship>,
}
