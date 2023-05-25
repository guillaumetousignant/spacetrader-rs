use super::CargoData;
use super::CrewData;
use super::EngineData;
use super::FrameData;
use super::FuelData;
use super::ModuleData;
use super::MountData;
use super::NavData;
use super::ReactorData;
use super::RegistrationData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShipData {
    symbol: String,
    nav: NavData,
    crew: CrewData,
    fuel: FuelData,
    frame: FrameData,
    reactor: ReactorData,
    engine: EngineData,
    modules: Vec<ModuleData>,
    mounts: Vec<MountData>,
    registration: RegistrationData,
    cargo: CargoData,
}
