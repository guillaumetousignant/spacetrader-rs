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
    trace!("Started contracts task");
    let mut interval = time::interval(time::Duration::from_secs(10));

    loop {
        interval.tick().await;
        trace!("Running contracts task");

        let contracts = queries::contracts(&client, &sender, &credentials.token).await?;
        for contract in contracts.iter() {
            if !contract.accepted {
                info!("Contract with id \"{}\" needs to be accepted", contract.id);
                queries::accept_contract(&client, &sender, &credentials.token, &contract.id)
                    .await?;
            }
        }
    }
}
