use super::Contract;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractResponse {
    pub data: Contract,
}
