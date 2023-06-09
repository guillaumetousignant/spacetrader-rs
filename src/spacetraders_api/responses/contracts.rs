use super::Contract;
use super::MetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contracts {
    pub data: Vec<Contract>,
    pub meta: MetaData,
}
