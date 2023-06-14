use super::Query;
use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Ships;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn ships_page(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Ships, Box<dyn std::error::Error + Send + Sync>> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    let request = client
        .get(format!("{URL}/my/ships"))
        .bearer_auth(token)
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
