use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    symbol: String,
    name: String,
    description: String,
}
