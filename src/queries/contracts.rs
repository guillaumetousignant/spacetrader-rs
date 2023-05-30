use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::responses::Contracts;
use reqwest::Client;

pub async fn contracts(
    client: &Client,
    credentials: &Credentials,
) -> Result<Contracts, reqwest::Error> {
    client
        .get(format!("{URL}/my/contracts"))
        .bearer_auth(&credentials.token)
        .send()
        .await?
        .json()
        .await
}
