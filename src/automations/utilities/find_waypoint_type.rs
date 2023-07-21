use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::{System, Waypoint};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn find_waypoint_type_in_system(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    waypoint_type: &str,
    system: &System,
) -> Result<Option<Waypoint>, Box<dyn std::error::Error + Send + Sync>> {
    let system_response = queries::system(client, sender, token, system).await?;

    Ok(system_response
        .waypoints
        .iter()
        .find(|&waypoint| waypoint.waypoint_type == waypoint_type)
        .map(|loc| loc.symbol.clone()))
}
