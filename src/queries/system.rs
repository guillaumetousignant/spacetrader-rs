use super::Query;
use super::URL;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::System;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn system(
    client: &Client,
    sender: &Sender<Query>,
    token: impl Into<Option<&str>>,
    system: &System,
) -> Result<responses::System, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.into();
    let request = client
        .get(format!("{URL}/systems/{system}"))
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
        .json::<responses::SystemResponse>()
        .await?
        .data)
}
