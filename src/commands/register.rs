use crate::helpers::Credentials;
use crate::queries;
use std::fs;
use std::path::Path;

pub async fn register(
    path: &Path,
    callsign: &str,
    faction: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let agent_response = queries::register(&client, callsign, faction).await?;

    let credentials = Credentials {
        token: agent_response.data.token.clone(),
    };

    println!("{:#?}", agent_response);

    fs::write(path, serde_json::to_string_pretty(&credentials)?)?;
    Ok(())
}
