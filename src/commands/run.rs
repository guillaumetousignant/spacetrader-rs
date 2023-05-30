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

    let location_response = queries::waypoint(
        &client,
        &agent_response.data.headquarters.system(),
        &agent_response.data.headquarters,
    )
    .await?;
    println!("Starting location: {:#?}", location_response);

    let waypoints_response =
        queries::waypoints(&client, &agent_response.data.headquarters.system()).await?;
    println!("Starting system location: {:#?}", waypoints_response);

    let system_response =
        queries::system(&client, &agent_response.data.headquarters.system()).await?;
    println!("Starting system: {:#?}", system_response);

    let systems_response = queries::systems(&client).await?;
    println!("Systems: {:#?}", systems_response);

    let factions_response = queries::factions(&client).await?;
    println!("Factions: {:#?}", factions_response);

    let faction_response = queries::faction(&client, &agent_response.data.starting_faction).await?;
    println!("Starting faction: {:#?}", faction_response);

    let contracts_response = queries::contracts(&client, &credentials).await?;
    println!("Contracts: {:#?}", contracts_response);

    for contract in contracts_response.data.iter() {
        let contract_response = queries::contract(&client, &credentials, &contract.id).await?;
        println!("Contract: {:#?}", contract_response);
        if !contract.accepted {
            println!("Contract with id \"{}\" needs to be accepted", contract.id);
            let contract_accept_response =
                queries::accept_contract(&client, &credentials, &contract.id).await?;
            println!("Contract accepted: {:#?}", contract_accept_response);
        }
    }

    let ships_response = queries::ships(&client, &credentials).await?;
    println!("Ships: {:#?}", ships_response);

    if ships_response.data.len() > 0 {
        let ship_response =
            queries::ship(&client, &credentials, &ships_response.data[0].symbol).await?;
        println!("Ship: {:#?}", ship_response);
    }

    Ok(())
}
