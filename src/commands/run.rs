use crate::helpers::Credentials;
use crate::queries;
use std::fs;
use std::path::Path;

pub async fn run(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let credentials_data = fs::read_to_string(path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let agent_response = queries::agent(&client, &credentials).await?;
    println!("{:#?}", agent_response);
    Ok(())
}
