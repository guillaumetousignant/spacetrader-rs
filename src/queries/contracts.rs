use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::helpers::Credentials;
use crate::spacetraders_api::requests::Page;
use crate::spacetraders_api::responses::Contracts;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn contracts(
    client: &Client,
    credentials: &Credentials,
    page: impl Into<Option<u128>>,
    limit: impl Into<Option<u128>>,
) -> Result<Contracts, Box<dyn std::error::Error>> {
    let page_query = Page {
        page: page.into(),
        limit: limit.into(),
    };

    for _ in 0..N_RETRIES {
        let response = client
            .get(format!("{URL}/my/contracts"))
            .bearer_auth(&credentials.token)
            .query(&page_query)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => return Ok(response.json().await?),
            StatusCode::TOO_MANY_REQUESTS => {
                let duration = get_rate_limit(&response)?;
                tokio::time::sleep(duration).await;
            }
            _ => {
                return Err(StatusError {
                    status: response.status(),
                    url: response.url().clone(),
                }
                .into())
            }
        }
    }

    Err(TooManyRetriesError.into())
}
