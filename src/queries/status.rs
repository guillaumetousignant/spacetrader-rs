use super::URL;
use crate::spacetraders_api::responses::Status;
use reqwest::Client;

pub async fn status(client: &Client) -> Result<Status, reqwest::Error> {
    client.get(format!("{URL}")).send().await?.json().await
}
