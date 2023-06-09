use super::Cargo;
use super::Crew;
use super::Engine;
use super::Frame;
use super::Fuel;
use super::Meta;
use super::Module;
use super::Mount;
use super::Nav;
use super::Reactor;
use super::Registration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub nav: Nav,
    pub crew: Crew,
    pub fuel: Fuel,
    pub frame: Frame,
    pub reactor: Reactor,
    pub engine: Engine,
    pub modules: Vec<Module>,
    pub mounts: Vec<Mount>,
    pub registration: Registration,
    pub cargo: Cargo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipResponse {
    pub data: Ship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ships {
    pub data: Vec<Ship>,
    pub meta: Meta,
}
