use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    pub symbol: String,
    pub faction: String,
}
