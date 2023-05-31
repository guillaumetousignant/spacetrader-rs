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

    let systems_response = queries::systems(&client, 1, 20).await?;
    println!("Systems: {:#?}", systems_response);

    let n_systems = systems_response.meta.total;
    let page_size = systems_response.meta.limit;
    let n_pages = (n_systems + (page_size - 1)) / page_size;

    for i in 2..=n_pages {
        let systems_response = queries::systems(&client, i, page_size).await?;
        println!("Systems: {:#?}", systems_response);
    }

    Ok(())
}
