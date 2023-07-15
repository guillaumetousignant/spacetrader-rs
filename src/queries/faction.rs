use super::Query;
use super::MAX_LIMIT;
use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Faction;
use crate::spacetraders_api::responses::FactionResponse;
use crate::spacetraders_api::responses::Factions;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn faction(
    client: &Client,
    sender: &Sender<Query>,
    faction: &str,
) -> Result<Faction, Box<dyn std::error::Error + Send + Sync>> {
    let request = client.get(format!("{URL}/factions/{faction}"));
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<FactionResponse>().await?.data)
}

pub async fn factions_page(
    client: &Client,
    sender: &Sender<Query>,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Factions, Box<dyn std::error::Error + Send + Sync>> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    let request = client.get(format!("{URL}/factions")).query(&page_query);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json().await?)
}

pub async fn factions(
    client: &Client,
    sender: &Sender<Query>,
) -> Result<Vec<Faction>, Box<dyn std::error::Error + Send + Sync>> {
    let response = factions_page(client, sender, 1, MAX_LIMIT).await?;
    let n_items = response.meta.total;
    let n_pages = (n_items + MAX_LIMIT - 1) / MAX_LIMIT;

    return match n_pages {
        1 => Ok(response.data),
        _ => {
            let mut data = response.data;
            for i in 2..n_pages {
                let mut response = factions_page(client, sender, i, MAX_LIMIT).await?;
                data.append(&mut response.data);
            }
            Ok(data)
        }
    };
}
