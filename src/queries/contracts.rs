use super::contracts_page;
use super::Query;
use super::MAX_LIMIT;
use crate::spacetraders_api::responses::Contract;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

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
