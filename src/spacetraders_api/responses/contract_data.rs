use super::TermsData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractData {
    pub id: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: TermsData,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: DateTime<Utc>,
    #[serde(rename = "deadlineToAccept")]
    pub deadline_to_accept: DateTime<Utc>,
}
