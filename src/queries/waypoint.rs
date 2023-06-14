use super::Query;
use super::URL;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn waypoint(
    client: &Client,
    sender: &Sender<Query>,
    token: impl Into<Option<&str>>,
    system: &System,
    waypoint: &Waypoint,
) -> Result<responses::Waypoint, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.into();
    let request = client
        .get(format!("{URL}/systems/{system}/waypoints/{waypoint}"))
        .bearer_auth(token.unwrap_or(""));
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx
        .await??
        .json::<responses::WaypointResponse>()
        .await?
        .data)
}
