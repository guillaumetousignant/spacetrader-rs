use crate::spacetraders_api::requests::Registration;
use crate::spacetraders_api::responses::AgentRegistration;
use crate::utilities::Credentials;
use std::fs;
use std::path::Path;

pub async fn register(
    path: &Path,
    callsign: &str,
    faction: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let registration_request = Registration {
        symbol: callsign.into(),
        faction: faction.into(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.spacetraders.io/v2/register")
        .json(&registration_request)
        .send()
        .await?;
    let agent_response: AgentRegistration = res.json().await?;

    let credentials = Credentials {
        token: agent_response.data.token.clone(),
    };

    println!("{:#?}", agent_response);

    fs::write(path, serde_json::to_string_pretty(&credentials)?)?;
    Ok(())
}
