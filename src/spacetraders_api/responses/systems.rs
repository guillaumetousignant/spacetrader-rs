use super::MetaData;
use super::SystemData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Systems {
    pub data: Vec<SystemData>,
    pub meta: MetaData,
}
