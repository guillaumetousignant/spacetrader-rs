use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Ships;
use reqwest::Client;

pub async fn ships(
    client: &Client,
    credentials: &Credentials,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Ships, reqwest::Error> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    client
        .get(format!("{URL}/my/ships"))
        .bearer_auth(&credentials.token)
        .query(&page_query)
        .send()
        .await?
        .json()
        .await
}
