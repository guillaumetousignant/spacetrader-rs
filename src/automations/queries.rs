use crate::queries::{
    get_rate_limit, Query, RequestNotClonedError, StatusError, TooManyRetriesError, N_RETRIES,
};
use log::{info, trace, warn};
use reqwest::StatusCode;
use tokio::sync::mpsc::Receiver;
use tokio::time;

pub async fn queries(
    mut receiver: Receiver<Query>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    trace!("Started queries task");
    let mut interval = time::interval(time::Duration::from_millis(500));

    while let Some(query) = receiver.recv().await {
        info!("Received query");

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
                match response.status() {
                    StatusCode::TOO_MANY_REQUESTS => {
                        warn!("Rate limited");
                        let duration = get_rate_limit(&response)?;
                        if i + 1 >= N_RETRIES {
                            let _ = query.response.send(Err(TooManyRetriesError.into()));
                            break;
                        }
                        tokio::time::sleep(duration).await;
                    }
                    _ => {
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
        }

        interval.tick().await;
    }
    Ok(())
}
