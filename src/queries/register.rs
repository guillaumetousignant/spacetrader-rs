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
        .post("https://api.spacetraders.io/v2/register")
        .json(&registration_request)
        .send()
        .await?
        .json()
        .await
}
