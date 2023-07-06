use crate::automations;
use crate::automations::ShipAutomation;
use crate::local_data;
use crate::queries::Query;
use crate::spacetraders_api::Waypoint;
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
    let mut interval = time::interval(time::Duration::from_secs(10));

    loop {
        interval.tick().await;
        trace!("Running acquisitions task");

        let ships = fs::read_to_string(ships_path).await?;
        let mut ships: local_data::Ships = serde_json::from_str(&ships)?;
        let mut n_mining_ships: u128 = 0;
        for ship in ships.ships.iter() {
            if let ShipAutomation::Mining = ship.automation {
                n_mining_ships += 1;
            }
        }

        if n_mining_ships == 0 {
            // Buy ship
            info!("No mining ship, purchasing mining ship");

            const SHIP_TYPE: &str = "SHIP_MINING_DRONE";

            let shipyard_waypoint =
                get_shipyard(&client, &sender, &credentials.token, &ships, &SHIP_TYPE).await?;

            match shipyard_waypoint {
                Some(waypoint) => {
                    let ship_purchase = queries::purchase_ship(
                        &client,
                        &sender,
                        &credentials.token,
                        &SHIP_TYPE,
                        &waypoint,
                    )
                    .await?;

                    info!(
                        "Purchased ship of type {SHIP_TYPE} with symbol {}",
                        ship_purchase.ship.symbol
                    );

                    let symbol_mining = ship_purchase.ship.symbol.clone();

                    ships.ships.push(local_data::Ship {
                        symbol: ship_purchase.ship.symbol,
                        automation: ShipAutomation::Mining,
                    });

                    fs::write(ships_path, serde_json::to_string_pretty(&ships)?).await?;

                    spawn_ship_task(
                        client.clone(),
                        sender.clone(),
                        credentials.clone(),
                        symbol_mining,
                    );
                }
                None => warn!("No shipyard with probe present"),
            }
        }
    }
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

async fn get_shipyard(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ships: &local_data::Ships,
    ship_type: &str,
) -> Result<Option<Waypoint>, Box<dyn std::error::Error + Send + Sync>> {
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
                        for ship_type_iter in shipyard_response.ship_types.iter() {
                            if ship_type_iter.ship_type == ship_type {
                                return Ok(Some(waypoint.symbol));
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    Ok(None)
}
