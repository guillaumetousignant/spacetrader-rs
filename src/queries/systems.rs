use super::URL;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses;
use reqwest::Client;

pub async fn systems(
    client: &Client,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<responses::Systems, reqwest::Error> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    client
        .get(format!("{URL}/systems"))
        .query(&page_query)
        .send()
        .await?
        .json()
        .await
}
