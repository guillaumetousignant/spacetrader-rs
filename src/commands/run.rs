use crate::spacetraders_api::responses::Agent;
use crate::utilities::Credentials;
use std::fs;
use std::path::Path;

pub async fn run(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let credentials_data = fs::read_to_string(path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let res = client
        .get("https://api.spacetraders.io/v2/my/agent")
        .bearer_auth(credentials.token)
        .send()
        .await?;
    let agent_response: Agent = res.json().await?;
    println!("{:#?}", agent_response);
    Ok(())
}
