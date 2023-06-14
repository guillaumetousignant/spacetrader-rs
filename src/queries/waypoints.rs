use super::waypoints_page;
use super::Query;
use super::MAX_LIMIT;
use crate::spacetraders_api::responses::Waypoint;
use crate::spacetraders_api::System;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn waypoints(
    client: &Client,
    sender: &Sender<Query>,
    token: impl Into<Option<&str>>,
    system: &System,
) -> Result<Vec<Waypoint>, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.into();
    let response = waypoints_page(client, sender, token, system, 1, MAX_LIMIT).await?;
    let n_items = response.meta.total;
    let n_pages = (n_items + MAX_LIMIT - 1) / MAX_LIMIT;

    return match n_pages {
        1 => Ok(response.data),
        _ => {
            let mut data = response.data;
            for i in 2..n_pages {
                let mut response =
                    waypoints_page(client, sender, token, system, i, MAX_LIMIT).await?;
                data.append(&mut response.data);
            }
            Ok(data)
        }
    };
}
