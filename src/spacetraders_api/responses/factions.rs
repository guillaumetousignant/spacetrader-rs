use super::Faction;
use super::MetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factions {
    pub data: Vec<Faction>,
    pub meta: MetaData,
}
