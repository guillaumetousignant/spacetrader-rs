use super::Query;
use super::MAX_LIMIT;
use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Contract;
use crate::spacetraders_api::responses::ContractResponse;
use crate::spacetraders_api::responses::Contracts;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn contract(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    contract_id: &str,
) -> Result<Contract, Box<dyn std::error::Error + Send + Sync>> {
    let request = client
        .get(format!("{URL}/my/contracts/{contract_id}"))
        .bearer_auth(token);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<ContractResponse>().await?.data)
}

pub async fn contracts_page(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Contracts, Box<dyn std::error::Error + Send + Sync>> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    let request = client
        .get(format!("{URL}/my/contracts"))
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

pub async fn contracts(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
) -> Result<Vec<Contract>, Box<dyn std::error::Error + Send + Sync>> {
    let response = contracts_page(client, sender, token, 1, MAX_LIMIT).await?;
    let n_items = response.meta.total;
    let n_pages = (n_items + MAX_LIMIT - 1) / MAX_LIMIT;

    return match n_pages {
        1 => Ok(response.data),
        _ => {
            let mut data = response.data;
            for i in 2..n_pages {
                let mut response = contracts_page(client, sender, token, i, MAX_LIMIT).await?;
                data.append(&mut response.data);
            }
            Ok(data)
        }
    };
}
