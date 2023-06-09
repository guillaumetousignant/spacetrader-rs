use super::systems_page;
use super::MAX_LIMIT;
use crate::spacetraders_api::responses::System;
use reqwest::Client;

pub async fn systems(
    client: &Client,
    token: impl Into<Option<&str>>,
) -> Result<Vec<System>, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.into();
    let response = systems_page(client, token, 1, MAX_LIMIT).await?;
    let n_items = response.meta.total;
    let n_pages = (n_items + MAX_LIMIT - 1) / MAX_LIMIT;

    return match n_pages {
        1 => Ok(response.data),
        _ => {
            let mut data = response.data;
            for i in 2..n_pages {
                let mut response = systems_page(client, token, i, MAX_LIMIT).await?;
                data.append(&mut response.data);
            }
            Ok(data)
        }
    };
}
