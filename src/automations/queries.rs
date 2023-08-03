use super::{RATE_LIMIT_BUFFER_MILLI, RATE_LIMIT_MILLI};
use crate::automations::utilities::wait_until;
use crate::queries::{Query, RequestNotClonedError, StatusError, TooManyRetriesError, N_RETRIES};
use crate::spacetraders_api::errors::RateLimitResponse;
use log::{info, trace, warn};
use reqwest::StatusCode;
use tokio::sync::mpsc::Receiver;
use tokio::time;

pub async fn queries(
    mut receiver: Receiver<Query>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Started queries task");

    let mut interval = time::interval(time::Duration::from_millis(
        RATE_LIMIT_MILLI + RATE_LIMIT_BUFFER_MILLI,
    ));
    interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

    while let Some(query) = receiver.recv().await {
        trace!("Received query");

        for i in 0..N_RETRIES {
            let response = query
                .request
                .try_clone()
                .ok_or(RequestNotClonedError)?
                .send()
                .await?;

            if response.status().is_success() {
                let _ = query.response.send(Ok(response)); // We ignore errors here, because it just means that the other end doesn't care about the response
                break;
            } else {
                if response.status() == StatusCode::TOO_MANY_REQUESTS {
                    let expiration = response.json::<RateLimitResponse>().await?.error.data.reset;

                    warn!("Rate limited until {}, retry {}", expiration, i);
                    if i + 1 >= N_RETRIES {
                        let _ = query.response.send(Err(TooManyRetriesError.into()));
                        break;
                    }
                    wait_until(expiration).await?;
                } else if response.status().is_server_error() {
                    warn!(
                        "Server error \"{}\", retry {}",
                        response.status().canonical_reason().unwrap_or(""),
                        i
                    );
                    if i + 1 >= N_RETRIES {
                        let _ = query.response.send(Err(TooManyRetriesError.into()));
                        break;
                    }
                } else {
                    let _ = query.response.send(Err(StatusError {
                        status: response.status(),
                        url: response.url().clone(),
                        message: response.text().await?,
                    }
                    .into()));
                    break;
                }
            }
        }

        interval.tick().await;
    }
    Ok(())
}
