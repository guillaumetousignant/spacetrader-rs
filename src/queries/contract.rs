use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::responses::Contract;
use reqwest::Client;

pub async fn contract(
    client: &Client,
    credentials: &Credentials,
    contract_id: &str,
) -> Result<Contract, reqwest::Error> {
    client
        .get(format!("{URL}/my/contracts/{contract_id}"))
        .bearer_auth(&credentials.token)
        .send()
        .await?
        .json()
        .await
}
