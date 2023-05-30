use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::responses::Ships;
use reqwest::Client;

pub async fn ships(client: &Client, credentials: &Credentials) -> Result<Ships, reqwest::Error> {
    client
        .get(format!("{URL}/my/ships"))
        .bearer_auth(&credentials.token)
        .send()
        .await?
        .json()
        .await
}
