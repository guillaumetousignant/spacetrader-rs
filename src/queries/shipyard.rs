use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::spacetraders_api::responses::Shipyard;
use crate::spacetraders_api::responses::ShipyardResponse;
use crate::spacetraders_api::responses::ShipyardUnauthorized;
use crate::spacetraders_api::responses::ShipyardUnauthorizedResponse;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn shipyard(
    client: &Client,
    token: &str,
    system: &System,
    waypoint: &Waypoint,
) -> Result<Shipyard, Box<dyn std::error::Error + Send + Sync>> {
    for _ in 0..N_RETRIES {
        let response = client
            .get(format!(
                "{URL}/systems/{system}/waypoints/{waypoint}/shipyard"
            ))
            .bearer_auth(token)
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => return Ok(response.json::<ShipyardResponse>().await?.data),
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

pub async fn shipyard_unauthorized(
    client: &Client,
    system: &System,
    waypoint: &Waypoint,
) -> Result<ShipyardUnauthorized, Box<dyn std::error::Error + Send + Sync>> {
    for _ in 0..N_RETRIES {
        let response = client
            .get(format!(
                "{URL}/systems/{system}/waypoints/{waypoint}/shipyard"
            ))
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => {
                return Ok(response.json::<ShipyardUnauthorizedResponse>().await?.data)
            }
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
