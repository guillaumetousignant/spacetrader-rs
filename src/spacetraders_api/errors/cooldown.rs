use crate::spacetraders_api::responses;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cooldown {
    pub message: String,
    pub code: i128,
    pub data: CooldownData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooldownResponse {
    pub error: Cooldown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CooldownData {
    pub cooldown: responses::Cooldown,
}
