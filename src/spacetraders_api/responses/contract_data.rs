use super::TermsData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractData {
    id: String,
    #[serde(rename = "factionSymbol")]
    faction_symbol: String,
    #[serde(rename = "type")]
    contract_type: String,
    terms: TermsData,
    accepted: bool,
    fulfilled: bool,
    expiration: DateTime<Utc>,
    #[serde(rename = "deadlineToAccept")]
    deadline_to_accept: DateTime<Utc>,
}
