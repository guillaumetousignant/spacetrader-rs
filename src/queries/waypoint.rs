use super::URL;
use crate::spacetraders_api::responses;
use crate::spacetraders_api::System;
use crate::spacetraders_api::Waypoint;
use reqwest::Client;

pub async fn waypoint(
    client: &Client,
    system: &System,
    waypoint: &Waypoint,
) -> Result<responses::Waypoint, reqwest::Error> {
    client
        .get(format!("{URL}/systems/{system}/waypoints/{waypoint}"))
        .send()
        .await?
        .json()
        .await
}
