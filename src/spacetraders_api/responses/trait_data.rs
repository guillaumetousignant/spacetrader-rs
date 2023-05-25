use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TraitData {
    symbol: String,
    name: String,
    description: String,
}
