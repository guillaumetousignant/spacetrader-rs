use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub symbol: String,
    pub faction: String,
}
