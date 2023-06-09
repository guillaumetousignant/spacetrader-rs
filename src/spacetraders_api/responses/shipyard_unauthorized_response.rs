use super::ShipyardUnauthorized;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipyardUnauthorizedResponse {
    pub data: ShipyardUnauthorized,
}
