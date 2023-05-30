use super::URL;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::System;
use reqwest::Client;

pub async fn system(client: &Client, system: &System) -> Result<responses::System, reqwest::Error> {
    client
        .get(format!("{URL}/systems/{system}"))
        .send()
        .await?
        .json()
        .await
}
