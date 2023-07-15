use super::Query;
use super::MAX_LIMIT;
use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::responses::Waypoints;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn waypoint(
    client: &Client,
    sender: &Sender<Query>,
    token: impl Into<Option<&str>>,
    system: &System,
    waypoint: &Waypoint,
) -> Result<responses::Waypoint, Box<dyn std::error::Error + Send + Sync>> {
    let token = token.into();
    let request = client
        .get(format!("{URL}/systems/{system}/waypoints/{waypoint}"))
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
        .json::<responses::WaypointResponse>()
        .await?
        .data)
}

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

pub async fn waypoints(
    client: &Client,
    sender: &Sender<Query>,
    token: impl Into<Option<&str>>,
    system: &System,
) -> Result<Vec<responses::Waypoint>, Box<dyn std::error::Error + Send + Sync>> {
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
