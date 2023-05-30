use super::ContractData;
use super::MetaData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contracts {
    pub data: Vec<ContractData>,
    pub meta: MetaData,
}
