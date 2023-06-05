use crate::helpers::Credentials;
use crate::queries;
use std::fs;
use std::path::Path;

pub async fn info(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let credentials_data = fs::read_to_string(path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let agent_response = queries::agent(&client, &credentials).await?;
    println!("Agent: {:#?}", agent_response);

    let contracts_response = queries::contracts(&client, &credentials, None, None).await?;
    println!("Contracts: {:#?}", contracts_response);

    let ships_response = queries::ships(&client, &credentials, None, None).await?;
    println!("Ships: {:#?}", ships_response);

    Ok(())
}
