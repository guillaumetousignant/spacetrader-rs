use super::Nav;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Orbit {
    pub nav: Nav,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitResponse {
    pub data: Orbit,
}
