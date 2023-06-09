use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::spacetraders_api::responses::ContractAccept;
use crate::spacetraders_api::responses::ContractAcceptResponse;
use reqwest::header::CONTENT_LENGTH;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn accept_contract(
    client: &Client,
    token: &str,
    contract_id: &str,
) -> Result<ContractAccept, Box<dyn std::error::Error + Send + Sync>> {
    for _ in 0..N_RETRIES {
        let response = client
            .post(format!("{URL}/my/contracts/{contract_id}/accept"))
            .bearer_auth(token)
            .header(CONTENT_LENGTH, 0)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => return Ok(response.json::<ContractAcceptResponse>().await?.data),
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
