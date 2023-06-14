use super::Query;
use super::URL;
use crate::spacetraders_api::requests::Registration;
use crate::spacetraders_api::responses::AgentRegistration;
use crate::spacetraders_api::responses::AgentRegistrationResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn register(
    client: &Client,
    sender: &Sender<Query>,
    callsign: &str,
    faction: &str,
) -> Result<AgentRegistration, Box<dyn std::error::Error + Send + Sync>> {
    let registration_request = Registration {
        symbol: callsign.into(),
        faction: faction.into(),
    };

    let request = client
        .post(format!("{URL}/register"))
        .json(&registration_request);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx
        .await??
        .json::<AgentRegistrationResponse>()
        .await?
        .data)
}
