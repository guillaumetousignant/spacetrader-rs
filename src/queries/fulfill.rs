use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Fulfill;
use crate::spacetraders_api::responses::FulfillResponse;
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn fulfill(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    contract_id: &str,
) -> Result<Fulfill, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .post(format!("{URL}/my/contracts/{contract_id}/fulfill"))
        .bearer_auth(token)
        .header(CONTENT_LENGTH, 0);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<FulfillResponse>().await?.data)
}
