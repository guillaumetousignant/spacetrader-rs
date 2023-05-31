use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsData {
    agents: u128,
    ships: u128,
    systems: u128,
    waypoints: u128,
}
