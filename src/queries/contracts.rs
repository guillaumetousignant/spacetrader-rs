use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Contracts;
use reqwest::Client;

pub async fn contracts(
    client: &Client,
    credentials: &Credentials,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Contracts, reqwest::Error> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    client
        .get(format!("{URL}/my/contracts"))
        .bearer_auth(&credentials.token)
        .query(&page_query)
        .send()
        .await?
        .json()
        .await
}
