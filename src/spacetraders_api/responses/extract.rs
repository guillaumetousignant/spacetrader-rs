use super::Cargo;
use super::Cooldown;
use super::Extraction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Extract {
    pub extraction: Extraction,
    pub cooldown: Cooldown,
    pub cargo: Cargo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractResponse {
    pub data: Extract,
}
