use crate::{local_data::Credentials, queries};
use log::{info, trace};
use tokio::time;

pub async fn contracts(
    credentials: Credentials,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    trace!("Started contracts task");
    let client = reqwest::Client::new();
    let mut interval = time::interval(time::Duration::from_secs(10));

    loop {
        interval.tick().await;
        trace!("Running contracts task");

        let contracts_response =
            queries::contracts_page(&client, &credentials.token, None, None).await?;
        for contract in contracts_response.data.iter() {
            if !contract.accepted {
                info!("Contract with id \"{}\" needs to be accepted", contract.id);
                queries::accept_contract(&client, &credentials.token, &contract.id).await?;
            }
        }
    }
}
