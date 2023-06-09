use crate::automations::acquisitions;
use crate::automations::contracts;
use crate::local_data::Credentials;
use crate::queries;
use log::info;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use tokio::task::JoinSet;

pub async fn run(
    credentials_path: &Path,
    ships_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let credentials_data = fs::read_to_string(credentials_path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let agent_response = queries::agent(&client, &credentials.token).await?;
    println!("Agent: {:#?}", agent_response);

    let mut set = JoinSet::new();

    let credentials_contracts = credentials.clone();
    set.spawn(async move { contracts(credentials_contracts).await });

    let acquisitions_contracts = credentials.clone();
    let acquisitions_ships_path = PathBuf::from(ships_path);
    set.spawn(async move { acquisitions(acquisitions_contracts, &acquisitions_ships_path).await });

    set.spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl-c signal received");
        Ok(())
    });

    set.join_next().await;

    Ok(())
}
