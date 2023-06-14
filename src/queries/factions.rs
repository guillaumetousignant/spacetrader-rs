use super::factions_page;
use super::Query;
use super::MAX_LIMIT;
use crate::spacetraders_api::responses::Faction;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

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
