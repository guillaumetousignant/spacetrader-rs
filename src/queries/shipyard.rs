use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Shipyard;
use crate::spacetraders_api::responses::ShipyardResponse;
use crate::spacetraders_api::responses::ShipyardUnauthorized;
use crate::spacetraders_api::responses::ShipyardUnauthorizedResponse;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn shipyard(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    system: &System,
    waypoint: &Waypoint,
) -> Result<Shipyard, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .get(format!(
            "{URL}/systems/{system}/waypoints/{waypoint}/shipyard"
        ))
        .bearer_auth(token);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<ShipyardResponse>().await?.data)
}

pub async fn shipyard_unauthorized(
    client: &Client,
    sender: &Sender<Query>,
    system: &System,
    waypoint: &Waypoint,
) -> Result<ShipyardUnauthorized, Box<dyn std::error::Error + Send + Sync>> {
    let request = client.get(format!(
        "{URL}/systems/{system}/waypoints/{waypoint}/shipyard"
    ));
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx
        .await??
        .json::<ShipyardUnauthorizedResponse>()
        .await?
        .data)
}
