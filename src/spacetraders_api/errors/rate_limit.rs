use crate::spacetraders_api::responses;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub message: String,
    pub code: i128,
    pub data: responses::RateLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResponse {
    pub error: RateLimit,
}
