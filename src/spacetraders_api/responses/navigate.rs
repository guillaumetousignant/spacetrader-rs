use super::Fuel;
use super::Nav;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navigate {
    pub nav: Nav,
    pub fuel: Fuel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigateResponse {
    pub data: Navigate,
}
