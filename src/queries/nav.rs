use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Nav;
use crate::spacetraders_api::responses::NavResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn nav(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_id: &str,
) -> Result<Nav, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .get(format!("{URL}/my/ships/{ship_id}/nav"))
        .bearer_auth(token);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<NavResponse>().await?.data)
}
