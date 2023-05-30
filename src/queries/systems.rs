use super::URL;
use crate::spacetraders_api::responses;
use reqwest::Client;

pub async fn systems(client: &Client) -> Result<responses::Systems, reqwest::Error> {
    client
        .get(format!("{URL}/systems"))
        .send()
        .await?
        .json()
        .await
}
