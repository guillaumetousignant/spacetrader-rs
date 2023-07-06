use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Orbit;
use crate::spacetraders_api::responses::OrbitResponse;
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn orbit(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_id: &str,
) -> Result<Orbit, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .post(format!("{URL}/my/ships/{ship_id}/orbit"))
        .bearer_auth(token)
        .header(CONTENT_LENGTH, 0);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<OrbitResponse>().await?.data)
}
