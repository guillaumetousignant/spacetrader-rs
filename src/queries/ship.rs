use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::spacetraders_api::responses::Ship;
use crate::spacetraders_api::responses::ShipResponse;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn ship(
    client: &Client,
    token: &str,
    ship_id: &str,
) -> Result<Ship, Box<dyn std::error::Error + Send + Sync>> {
    for _ in 0..N_RETRIES {
        let response = client
            .get(format!("{URL}/my/ships/{ship_id}"))
            .bearer_auth(token)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => return Ok(response.json::<ShipResponse>().await?.data),
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
