use crate::automations;
use crate::automations::ShipAutomation;
use crate::local_data;
use crate::local_data::Credentials;
use crate::queries;
use crate::queries::Query;
use log::info;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::task::JoinSet;

pub async fn run(
    credentials_path: &Path,
    ships_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let credentials_data = fs::read_to_string(credentials_path).await?;
    let credentials: Credentials = serde_json::from_str(&credentials_data)?;

    let mut set = JoinSet::new();

    // Queries task
    let (tx, rx) = mpsc::channel::<Query>(32);
    set.spawn(async move { automations::queries(rx).await });

    let client = reqwest::Client::new();

    // Print agent info
    let credentials_agent = credentials.clone();
    let client_agent = client.clone();
    let tx_agent = tx.clone();
    let task_agent: JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> =
        tokio::spawn(async move {
            let agent_response =
                queries::agent(&client_agent, &tx_agent, &credentials_agent.token).await?;
            println!("Agent: {:#?}", agent_response);
            Ok(())
        });
    task_agent.await??;

    // Contracts task
    let credentials_contracts = credentials.clone();
    let client_contracts = client.clone();
    let tx_contracts = tx.clone();
    set.spawn(async move {
        automations::contracts(client_contracts, tx_contracts, credentials_contracts).await
    });

    // Ship automations
    let ships = fs::read_to_string(ships_path).await?;
    let ships: local_data::Ships = serde_json::from_str(&ships)?;
    for ship in ships.ships {
        match ship.automation {
            ShipAutomation::Command => {}
            ShipAutomation::Mining => {
                let credentials_mining = credentials.clone();
                let client_mining = client.clone();
                let tx_mining = tx.clone();

                set.spawn(async move {
                    automations::mining(client_mining, tx_mining, credentials_mining, ship.symbol)
                        .await
                });
            }
            ShipAutomation::Probe => {}
            ShipAutomation::None => {}
        }
    }

    // Acquisitions task
    let credentials_acquisitions = credentials.clone();
    let client_acquisitions = client.clone();
    let tx_acquisitions = tx.clone();
    let ships_acquisitions = PathBuf::from(ships_path);
    set.spawn(async move {
        automations::acquisitions(
            client_acquisitions,
            tx_acquisitions,
            credentials_acquisitions,
            &ships_acquisitions,
        )
        .await
    });

    // Program interruption watch task
    set.spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        info!("Ctrl-c signal received");
        Ok(())
    });

    match set.join_next().await {
        Some(result) => result?,
        None => Ok(()),
    }
}
