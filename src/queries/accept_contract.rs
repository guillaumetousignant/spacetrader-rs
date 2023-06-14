use super::Query;
use super::URL;
use crate::spacetraders_api::responses::ContractAccept;
use crate::spacetraders_api::responses::ContractAcceptResponse;
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn accept_contract(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    contract_id: &str,
) -> Result<ContractAccept, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .post(format!("{URL}/my/contracts/{contract_id}/accept"))
        .bearer_auth(token)
        .header(CONTENT_LENGTH, 0);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<ContractAcceptResponse>().await?.data)
}
