use super::ContractData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub data: ContractData,
}
