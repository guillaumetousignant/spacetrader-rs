use crate::automations;
use crate::local_data::Credentials;
use crate::local_data::Ships;
use crate::queries;
use crate::queries::Query;
use std::path::Path;
use tokio::fs;
use tokio::sync::mpsc;

pub async fn info(
    credentials_path: &Path,
    ships_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Query>(32);
    tokio::spawn(async move { automations::queries(rx).await });

    let credentials_data = fs::read_to_string(credentials_path).await?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let agent_response = queries::agent(&client, &tx, &credentials.token).await?;
    println!("Agent: {:#?}", agent_response);

    let contracts_response = queries::contracts(&client, &tx, &credentials.token).await?;
    println!("Contracts: {:#?}", contracts_response);

    let ships_response = queries::ships(&client, &tx, &credentials.token).await?;
    println!("Ships: {:#?}", ships_response);

    let ships = fs::read_to_string(ships_path).await?;
    let ships: Ships = serde_json::from_str(&ships)?;

    println!("Ship automations:");
    for ship in ships.ships.iter() {
        println!("\t{}: {:?}", ship.symbol, ship.automation);
    }

    Ok(())
}
