use super::Agent;
use super::Ship;
use super::ShipyardTransaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseShip {
    pub agent: Agent,
    pub ship: Ship,
    pub transaction: ShipyardTransaction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseShipResponse {
    pub data: PurchaseShip,
}
