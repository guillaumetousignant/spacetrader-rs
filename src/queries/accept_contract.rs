use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::responses::ContractAccept;
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;

pub async fn accept_contract(
    client: &Client,
    credentials: &Credentials,
    contract_id: &str,
) -> Result<ContractAccept, reqwest::Error> {
    client
        .post(format!("{URL}/my/contracts/{contract_id}/accept"))
        .bearer_auth(&credentials.token)
        .header(CONTENT_LENGTH, 0)
        .send()
        .await?
        .json()
        .await
}
