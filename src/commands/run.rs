use crate::helpers::Credentials;
use crate::queries;
use std::fs;
use std::path::Path;

pub async fn run(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let credentials_data = fs::read_to_string(path)?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let client = reqwest::Client::new();
    let agent_response = queries::agent(&client, &credentials).await?;
    println!("Agent: {:#?}", agent_response);

    let contracts_response = queries::contracts(&client, &credentials, None, None).await?;
    for contract in contracts_response.data.iter() {
        if !contract.accepted {
            println!("Contract with id \"{}\" needs to be accepted", contract.id);
            queries::accept_contract(&client, &credentials, &contract.id).await?;
        }
    }

    let ships_response = queries::ships(&client, &credentials, None, None).await?;
    let mut mining_ship: Option<String> = None;
    for ship in ships_response.data {
        if ship.frame.symbol == "FRAME_DRONE" {
            mining_ship = Some(ship.symbol.clone());
            println!("Using {} as mining ship", ship.symbol);
            break;
        }
    }

    if let None = mining_ship {
        // Buy ship
        println!("Buying ship");
    }

    Ok(())
}
