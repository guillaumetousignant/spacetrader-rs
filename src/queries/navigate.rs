use super::Query;
use super::URL;
use crate::spacetraders_api::requests;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::responses::NavigateResponse;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn navigate(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_id: &str,
    waypoint: &Waypoint,
) -> Result<responses::Navigate, Box<dyn std::error::Error + Send + Sync>> {
    let request_data = requests::Navigate {
        waypoint_symbol: waypoint.clone(),
    };

    let request = client
        .post(format!("{URL}/my/ships/{ship_id}/navigate"))
        .bearer_auth(token)
        .json(&request_data);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<NavigateResponse>().await?.data)
}
