use crate::queries;
use crate::queries::Query;
use crate::spacetraders_api::responses::Trait;
use crate::spacetraders_api::{System, Waypoint};
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub fn find_trait<'a>(traits: &'a [Trait], to_find: &str) -> Option<&'a Trait> {
    traits
        .iter()
        .find(|&trait_iter| trait_iter.symbol == to_find)
}

pub async fn find_trait_in_system(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    trait_name: &str,
    system: &System,
) -> Result<Option<Waypoint>, Box<dyn std::error::Error + Send + Sync>> {
    let system_response = queries::waypoints(client, sender, token, system).await?;

    Ok(system_response
        .iter()
        .find(|&waypoint| find_trait(&waypoint.traits, trait_name).is_some())
        .map(|waypoint| waypoint.symbol.clone()))
}
