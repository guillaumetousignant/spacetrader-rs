use super::Nav;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavStatus {
    pub nav: Nav,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavStatusResponse {
    pub data: NavStatus,
}
