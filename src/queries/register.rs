use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::spacetraders_api::requests::Registration;
use crate::spacetraders_api::responses::AgentRegistration;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn register(
    client: &Client,
    callsign: &str,
    faction: &str,
) -> Result<AgentRegistration, Box<dyn std::error::Error>> {
    let registration_request = Registration {
        symbol: callsign.into(),
        faction: faction.into(),
    };

    for _ in 0..N_RETRIES {
        let response = client
            .post(format!("{URL}/register"))
            .json(&registration_request)
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
