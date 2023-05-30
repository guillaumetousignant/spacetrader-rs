use super::FactionData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Faction {
    data: FactionData,
}
