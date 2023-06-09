use crate::automations::ShipAutomation;
use crate::local_data::Ships;
use crate::{local_data::Credentials, queries};
use log::{info, trace};
use std::fs;
use std::path::Path;
use tokio::time;

pub async fn acquisitions(
    credentials: Credentials,
    ships_path: &Path,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    trace!("Started acquisitions task");
    let client = reqwest::Client::new();
    let mut interval = time::interval(time::Duration::from_secs(10));

    loop {
        interval.tick().await;
        trace!("Running acquisitions task");

        let ships = fs::read_to_string(ships_path)?;
        let ships: Ships = serde_json::from_str(&ships)?;
        let mut n_mining_ships: u128 = 0;
        for ship in ships.ships {
            if let ShipAutomation::Mining = ship.automation {
                n_mining_ships += 1;
            }
        }

        if n_mining_ships == 0 {
            // Buy ship
            info!("No mining ship, buying mining ship");
        }
    }
}
