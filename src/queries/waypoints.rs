use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Waypoints;
use crate::spacetraders_api::System;
use reqwest::Client;

pub async fn waypoints(
    client: &Client,
    system: &System,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Waypoints, reqwest::Error> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    client
        .get(format!("{URL}/systems/{system}/waypoints"))
        .query(&page_query)
        .send()
        .await?
        .json()
        .await
}
