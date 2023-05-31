mod accept_contract;
mod agent;
mod contract;
mod contracts;
mod faction;
mod factions;
mod register;
mod ship;
mod ships;
mod status;
mod system;
mod systems;
mod waypoint;
mod waypoints;

pub use accept_contract::accept_contract;
pub use agent::agent;
pub use contract::contract;
pub use contracts::contracts;
pub use faction::faction;
pub use factions::factions;
pub use register::register;
pub use ship::ship;
pub use ships::ships;
pub use status::status;
pub use system::system;
pub use systems::systems;
pub use waypoint::waypoint;
pub use waypoints::waypoints;

pub const URL: &str = "https://api.spacetraders.io/v2";
