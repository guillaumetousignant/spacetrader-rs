use super::Shipyard;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardResponse {
    pub data: Shipyard,
}
