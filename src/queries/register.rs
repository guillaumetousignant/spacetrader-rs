use super::URL;
use crate::spacetraders_api::requests::Registration;
use crate::spacetraders_api::responses::AgentRegistration;
use reqwest::Client;

pub async fn register(
    client: &Client,
    callsign: &str,
    faction: &str,
) -> Result<AgentRegistration, reqwest::Error> {
    let registration_request = Registration {
        symbol: callsign.into(),
        faction: faction.into(),
    };

    client
        .post(format!("{URL}/register"))
        .json(&registration_request)
        .send()
        .await?
        .json()
        .await
}
