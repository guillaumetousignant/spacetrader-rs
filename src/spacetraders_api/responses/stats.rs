use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    agents: u128,
    ships: u128,
    systems: u128,
    waypoints: u128,
}
