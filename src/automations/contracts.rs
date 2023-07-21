use super::CONTRACTS_INTERVAL_SECS;
use crate::queries::Query;
use crate::{local_data::Credentials, queries};
use log::{info, trace};
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::time;

pub async fn contracts(
    client: Client,
    sender: Sender<Query>,
    credentials: Credentials,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Started contracts task");

    let mut interval = time::interval(time::Duration::from_secs(CONTRACTS_INTERVAL_SECS));

    loop {
        interval.tick().await;
        trace!("Running contracts task");

        let contracts = queries::contracts(&client, &sender, &credentials.token).await?;
        for contract in contracts.iter() {
            if !contract.accepted {
                info!("Contract with id \"{}\" needs to be accepted", contract.id);
                let _ =
                    queries::accept_contract(&client, &sender, &credentials.token, &contract.id)
                        .await?;
            } else if !contract.fulfilled {
                for delivery in contract.terms.deliver.iter() {
                    if delivery.units_fulfilled < delivery.units_required {
                        break;
                    }
                }
                let unfulfilled_delivery = contract
                    .terms
                    .deliver
                    .iter()
                    .find(|&delivery| delivery.units_fulfilled < delivery.units_required)
                    .is_some();
                if !unfulfilled_delivery {
                    info!("Contract with id \"{}\" needs to be fulfilled", contract.id);
                    let _ = queries::fulfill(&client, &sender, &credentials.token, &contract.id)
                        .await?;
                }
            }
        }
    }
}
