use super::ACQUISITIONS_INTERVAL_SECS;
use crate::automations;
use crate::automations::ShipAutomation;
use crate::local_data;
use crate::queries::Query;
use crate::spacetraders_api::responses::{ShipType, Shipyard, ShipyardShip};
use crate::{local_data::Credentials, queries};
use log::{info, trace, warn};
use reqwest::Client;
use std::path::Path;
use tokio::fs;
use tokio::sync::mpsc::Sender;
use tokio::time;

pub async fn acquisitions(
    client: Client,
    sender: Sender<Query>,
    credentials: Credentials,
    ships_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Started acquisitions task");

    let mut interval = time::interval(time::Duration::from_secs(ACQUISITIONS_INTERVAL_SECS));

    const MINING_SHIP_TYPE: &str = "SHIP_MINING_DRONE";
    const MINING_SHIP_AUTOMATION: ShipAutomation = ShipAutomation::Mining;

    loop {
        interval.tick().await;
        trace!("Running acquisitions task");

        let ships = fs::read_to_string(ships_path).await?;
        let mut ships: local_data::Ships = serde_json::from_str(&ships)?;
        let mut n_mining_ships: u128 = 0;
        for ship in ships.ships.iter() {
            if let MINING_SHIP_AUTOMATION = ship.automation {
                n_mining_ships += 1;
            }
        }

        let shipyard = get_shipyard(
            &client,
            &sender,
            &credentials.token,
            &ships,
            &MINING_SHIP_TYPE,
        )
        .await?;

        if n_mining_ships == 0 {
            // Buy ship
            info!("No mining ship, purchasing mining ship");

            match &shipyard {
                Some(shipyard) => {
                    purchase_ship(&client, &sender, &credentials, ships_path, MINING_SHIP_TYPE, MINING_SHIP_AUTOMATION, shipyard, &mut ships).await?;
                    n_mining_ships += 1;
                }
                None => warn!("No shipyard with probe present and ship type {MINING_SHIP_TYPE} available to purchase."),
            }
        }

        match &shipyard {
            Some(shipyard) => {
                match get_shipyard_ship(&shipyard.ships, MINING_SHIP_TYPE) {
                    Some(ship) => {
                        loop {
                            let agent_response = queries::agent(&client, &sender, &credentials.token).await?;
                            let credits = agent_response.credits;
                            let purchase_price = ship.purchase_price;

                            if credits > 0 {
                                let credits = u128::try_from(credits)?;
                                if credits > purchase_price * (n_mining_ships + 1) {
                                    purchase_ship(&client, &sender, &credentials, ships_path, MINING_SHIP_TYPE, MINING_SHIP_AUTOMATION, shipyard, &mut ships).await?;
                                    n_mining_ships += 1;
                                }
                                else {
                                    break;
                                }
                            }
                            else {
                                break;
                            }
                        }
                    }
                    None => warn!("Shipyard {} has no ship type {MINING_SHIP_TYPE} available to purchase.", shipyard.symbol),
                }
            }
            None => warn!("No shipyard with probe present and ship type {MINING_SHIP_TYPE} available to purchase."),
        }
    }
}

async fn purchase_ship(
    client: &Client,
    sender: &Sender<Query>,
    credentials: &Credentials,
    ships_path: &Path,
    ship_type: &str,
    ship_automation: ShipAutomation,
    shipyard: &Shipyard,
    ships: &mut local_data::Ships,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ship_purchase = queries::purchase_ship(
        &client,
        &sender,
        &credentials.token,
        ship_type,
        &shipyard.symbol,
    )
    .await?;

    info!(
        "Purchased ship of type {ship_type} with symbol {}",
        ship_purchase.ship.symbol
    );

    let symbol_mining = ship_purchase.ship.symbol.clone();

    ships.ships.push(local_data::Ship {
        symbol: ship_purchase.ship.symbol,
        automation: ship_automation,
    });

    fs::write(ships_path, serde_json::to_string_pretty(&ships)?).await?;

    spawn_ship_task(
        client.clone(),
        sender.clone(),
        credentials.clone(),
        symbol_mining,
    );
    Ok(())
}

fn spawn_ship_task(
    client: Client,
    sender: Sender<Query>,
    credentials: Credentials,
    ship_symbol: String,
) {
    tokio::spawn(
        async move { automations::mining(client, sender, credentials, ship_symbol).await },
    );
}

fn get_ship_type<'a>(ship_types: &'a [ShipType], ship_type: &str) -> Option<&'a ShipType> {
    ship_types
        .iter()
        .find(|&ship_type_it| ship_type_it.ship_type == ship_type)
}

fn get_shipyard_ship<'a>(ships: &'a [ShipyardShip], ship_type: &str) -> Option<&'a ShipyardShip> {
    ships.iter().find(|&ship| ship.ship_type == ship_type)
}

async fn get_shipyard(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ships: &local_data::Ships,
    ship_type: &str,
) -> Result<Option<Shipyard>, Box<dyn std::error::Error + Send + Sync>> {
    for ship in ships.ships.iter() {
        if let ShipAutomation::Probe = ship.automation {
            let ship_response = queries::ship(client, sender, token, &ship.symbol).await?;
            if ship_response.nav.status == "DOCKED" {
                let waypoint = queries::waypoint(
                    client,
                    sender,
                    token,
                    &ship_response.nav.system_symbol,
                    &ship_response.nav.waypoint_symbol,
                )
                .await?;
                for trait_iter in waypoint.traits.iter() {
                    if trait_iter.symbol == "SHIPYARD" {
                        let shipyard_response = queries::shipyard(
                            client,
                            sender,
                            token,
                            &waypoint.system_symbol,
                            &waypoint.symbol,
                        )
                        .await?;

                        if get_ship_type(&shipyard_response.ship_types, ship_type).is_some() {
                            return Ok(Some(shipyard_response));
                        }
                        break;
                    }
                }
            }
        }
    }
    Ok(None)
}
