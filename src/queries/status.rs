use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Status;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn status(
    client: &Client,
    sender: &Sender<Query>,
) -> Result<Status, Box<dyn std::error::Error + Send + Sync>> {
    let request = client.get(format!("{URL}"));
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json().await?)
}
