use super::Query;
use super::URL;
use crate::spacetraders_api::requests;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::responses::DeliverResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn deliver(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    contract_id: &str,
    ship_id: &str,
    trade_symbol: &str,
    units: u128,
) -> Result<responses::Deliver, Box<dyn std::error::Error + Send + Sync>> {
    let request_data = requests::Deliver {
        ship_symbol: ship_id.to_owned(),
        trade_symbol: trade_symbol.to_owned(),
        units,
    };

    let request = client
        .post(format!("{URL}/my/contracts/{contract_id}/deliver"))
        .bearer_auth(token)
        .json(&request_data);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<DeliverResponse>().await?.data)
}
