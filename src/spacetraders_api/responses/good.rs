use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Good {
    pub symbol: String,
    pub name: String,
    pub description: String,
}
