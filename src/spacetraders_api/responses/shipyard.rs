use super::ShipyardData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Shipyard {
    pub data: ShipyardData,
}
