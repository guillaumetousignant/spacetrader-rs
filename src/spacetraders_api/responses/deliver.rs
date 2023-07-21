use super::Cargo;
use super::Contract;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliver {
    pub contract: Contract,
    pub cargo: Cargo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverResponse {
    pub data: Deliver,
}
