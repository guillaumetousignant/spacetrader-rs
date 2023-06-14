use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Faction;
use crate::spacetraders_api::responses::FactionResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn faction(
    client: &Client,
    sender: &Sender<Query>,
    faction: &str,
) -> Result<Faction, Box<dyn std::error::Error + Send + Sync>> {
    let request = client.get(format!("{URL}/factions/{faction}"));
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<FactionResponse>().await?.data)
}
