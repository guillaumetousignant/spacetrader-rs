use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::spacetraders_api::responses::Agent;
use crate::spacetraders_api::responses::AgentResponse;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn agent(
    client: &Client,
    token: &str,
) -> Result<Agent, Box<dyn std::error::Error + Send + Sync>> {
    for _ in 0..N_RETRIES {
        let response = client
            .get(format!("{URL}/my/agent"))
            .bearer_auth(token)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => return Ok(response.json::<AgentResponse>().await?.data),
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
