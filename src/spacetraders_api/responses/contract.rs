use super::Meta;
use super::Terms;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: DateTime<Utc>,
    #[serde(rename = "deadlineToAccept")]
    pub deadline_to_accept: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractResponse {
    pub data: Contract,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contracts {
    pub data: Vec<Contract>,
    pub meta: Meta,
}
