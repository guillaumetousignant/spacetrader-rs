use crate::local_data::Credentials;
use crate::local_data::Ships;
use crate::queries;
use std::fs;
use std::path::Path;

pub async fn info(
    credentials_path: &Path,
    ships_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let credentials_data = fs::read_to_string(credentials_path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let agent_response = queries::agent(&client, &credentials.token).await?;
    println!("Agent: {:#?}", agent_response);

    let contracts_response =
        queries::contracts_page(&client, &credentials.token, None, None).await?;
    println!("Contracts: {:#?}", contracts_response);

    let ships_response = queries::ships_page(&client, &credentials.token, None, None).await?;
    println!("Ships: {:#?}", ships_response);

    let ships = fs::read_to_string(ships_path)?;
    let ships: Ships = serde_json::from_str(&ships)?;

    println!("Ship automations:");
    for ship in ships.ships {
        println!("\t{}: {:?}", ship.symbol, ship.automation);
    }

    Ok(())
}
