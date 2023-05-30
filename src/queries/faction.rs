use super::URL;
use crate::spacetraders_api::responses::Faction;
use reqwest::Client;

pub async fn faction(client: &Client, faction: &str) -> Result<Faction, reqwest::Error> {
    client
        .get(format!("{URL}/factions/{faction}"))
        .send()
        .await?
        .json()
        .await
}
