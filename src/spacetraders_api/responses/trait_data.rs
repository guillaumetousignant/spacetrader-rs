use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}
