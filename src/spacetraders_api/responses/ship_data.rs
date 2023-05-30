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
    pub symbol: String,
    pub nav: NavData,
    pub crew: CrewData,
    pub fuel: FuelData,
    pub frame: FrameData,
    pub reactor: ReactorData,
    pub engine: EngineData,
    pub modules: Vec<ModuleData>,
    pub mounts: Vec<MountData>,
    pub registration: RegistrationData,
    pub cargo: CargoData,
}
