use super::URL;
use crate::spacetraders_api::responses::Waypoints;
use crate::spacetraders_api::System;
use reqwest::Client;

pub async fn waypoints(client: &Client, system: &System) -> Result<Waypoints, reqwest::Error> {
    client
        .get(format!("{URL}/systems/{system}/waypoints"))
        .send()
        .await?
        .json()
        .await
}
