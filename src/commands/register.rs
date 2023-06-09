use crate::automations::ShipAutomation;
use crate::automations::UnknownShipAutomationError;
use crate::local_data::{Credentials, Ship, Ships};
use crate::queries;
use std::fs;
use std::path::Path;

pub async fn register(
    credentials_path: &Path,
    ships_path: &Path,
    callsign: &str,
    faction: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::new();
    let agent_response = queries::register(&client, callsign, faction).await?;

    let credentials = Credentials {
        token: agent_response.token.clone(),
    };

    println!("{:#?}", agent_response);

    fs::write(
        credentials_path,
        serde_json::to_string_pretty(&credentials)?,
    )?;

    let mut ships: Vec<Ship> = vec![];
    let ships_response = queries::ships(&client, &credentials.token).await?;
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

    fs::write(ships_path, serde_json::to_string_pretty(&ships)?)?;

    Ok(())
}
