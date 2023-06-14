use super::Query;
use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Waypoints;
use crate::spacetraders_api::System;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn waypoints_page(
    client: &Client,
    sender: &Sender<Query>,
    token: impl Into<Option<&str>>,
    system: &System,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Waypoints, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.into();
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    let request = client
        .get(format!("{URL}/systems/{system}/waypoints"))
        .bearer_auth(token.unwrap_or(""))
        .query(&page_query);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json().await?)
}
