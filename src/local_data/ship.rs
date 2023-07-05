use crate::automations::ShipAutomation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub automation: ShipAutomation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ships {
    pub ships: Vec<Ship>,
}
