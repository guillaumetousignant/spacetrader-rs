use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Refuel;
use crate::spacetraders_api::responses::RefuelResponse;
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn refuel(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_id: &str,
) -> Result<Refuel, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .post(format!("{URL}/my/ships/{ship_id}/refuel"))
        .bearer_auth(token)
        .header(CONTENT_LENGTH, 0);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<RefuelResponse>().await?.data)
}
