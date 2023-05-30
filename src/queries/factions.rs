use super::URL;
use crate::spacetraders_api::responses::Factions;
use reqwest::Client;

pub async fn factions(client: &Client) -> Result<Factions, reqwest::Error> {
    client
        .get(format!("{URL}/factions"))
        .send()
        .await?
        .json()
        .await
}
