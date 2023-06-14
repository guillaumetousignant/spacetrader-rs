use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Ship;
use crate::spacetraders_api::responses::ShipResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn ship(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_id: &str,
) -> Result<Ship, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .get(format!("{URL}/my/ships/{ship_id}"))
        .bearer_auth(token);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<ShipResponse>().await?.data)
}
