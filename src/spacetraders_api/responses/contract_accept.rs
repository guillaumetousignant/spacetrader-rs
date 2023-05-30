use super::ContractAcceptData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractAccept {
    pub data: ContractAcceptData,
}
