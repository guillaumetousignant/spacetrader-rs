use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Market;
use crate::spacetraders_api::responses::MarketResponse;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn market(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    system: &System,
    waypoint: &Waypoint,
) -> Result<Market, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .get(format!(
            "{URL}/systems/{system}/waypoints/{waypoint}/market"
        ))
        .bearer_auth(token);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<MarketResponse>().await?.data)
}
