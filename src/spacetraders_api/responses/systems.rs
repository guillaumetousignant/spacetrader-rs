use super::MetaData;
use super::System;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Systems {
    pub data: Vec<System>,
    pub meta: MetaData,
}
