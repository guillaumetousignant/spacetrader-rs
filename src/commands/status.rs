use crate::automations;
use crate::queries;
use crate::queries::Query;
use tokio::sync::mpsc;

pub async fn status() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (tx, rx) = mpsc::channel::<Query>(32);
    tokio::spawn(async move { automations::queries(rx).await });

    let client = reqwest::Client::new();
    let status_response = queries::status(&client, &tx).await?;
    println!("{:#?}", status_response);

    Ok(())
}
