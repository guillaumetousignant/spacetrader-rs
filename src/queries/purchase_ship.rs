use super::Query;
use super::URL;
use crate::spacetraders_api::requests;
use crate::spacetraders_api::responses::PurchaseShip;
use crate::spacetraders_api::responses::PurchaseShipResponse;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn purchase_ship(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_type: &str,
    waypoint: &Waypoint,
) -> Result<PurchaseShip, Box<dyn std::error::Error + Send + Sync>> {
    let purchase_request = requests::PurchaseShip {
        ship_type: ship_type.into(),
        waypoint_symbol: waypoint.clone(),
    };

    let request = client
        .post(format!("{URL}/my/ships"))
        .bearer_auth(token)
        .json(&purchase_request);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<PurchaseShipResponse>().await?.data)
}
