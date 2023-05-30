use super::FactionData;
use super::MetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Factions {
    pub data: Vec<FactionData>,
    pub meta: MetaData,
}
