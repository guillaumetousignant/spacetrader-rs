use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::responses::Ship;
use reqwest::Client;

pub async fn ship(
    client: &Client,
    credentials: &Credentials,
    ship_id: &str,
) -> Result<Ship, reqwest::Error> {
    client
        .get(format!("{URL}/my/ships/{ship_id}"))
        .bearer_auth(&credentials.token)
        .send()
        .await?
        .json()
        .await
}
