use crate::automations;
use crate::automations::ShipAutomation;
use crate::automations::UnknownShipAutomationError;
use crate::local_data::{Credentials, Ship, Ships};
use crate::queries;
use crate::queries::Query;
use std::path::Path;
use tokio::fs;
use tokio::sync::mpsc;

pub async fn register(
    credentials_path: &Path,
    ships_path: &Path,
    callsign: &str,
    faction: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Query>(32);
    tokio::spawn(async move { automations::queries(rx).await });

    let client = reqwest::Client::new();
    let agent_response = queries::register(&client, &tx, callsign, faction).await?;

    let credentials = Credentials {
        token: agent_response.token.clone(),
    };

    println!("{:#?}", agent_response);

    fs::write(
        credentials_path,
        serde_json::to_string_pretty(&credentials)?,
    )
    .await?;

    let mut ships: Vec<Ship> = vec![];
    let ships_response = queries::ships(&client, &tx, &credentials.token).await?;
    for ship in ships_response {
        let automation = match ship.registration.role.as_str() {
            "COMMAND" => ShipAutomation::Command,
            "SURVEYOR" => ShipAutomation::Probe,
            _ => {
                return Err(UnknownShipAutomationError {
                    value: ship.registration.role,
                }
                .into())
            }
        };
        ships.push(Ship {
            symbol: ship.symbol,
            automation,
        })
    }

    let ships = Ships { ships };

    fs::write(ships_path, serde_json::to_string_pretty(&ships)?).await?;

    Ok(())
}
