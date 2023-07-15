use super::Query;
use super::URL;
use crate::spacetraders_api::requests;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::responses::SellResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn sell(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_id: &str,
    symbol: &str,
    units: u128,
) -> Result<responses::Sell, Box<dyn std::error::Error + Send + Sync>> {
    let request_data = requests::Sell {
        symbol: symbol.to_owned(),
        units,
    };

    let request = client
        .post(format!("{URL}/my/ships/{ship_id}/sell"))
        .bearer_auth(token)
        .json(&request_data);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<SellResponse>().await?.data)
}
