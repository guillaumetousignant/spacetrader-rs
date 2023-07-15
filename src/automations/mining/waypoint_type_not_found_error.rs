use crate::spacetraders_api::System;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct WaypointTypeNotFoundError {
    pub ship_symbol: String,
    pub waypoint_type: String,
    pub system: System,
}

impl fmt::Display for WaypointTypeNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} found no waypoint of type {} in system {}",
            self.ship_symbol, self.waypoint_type, self.system
        )
    }
}

impl error::Error for WaypointTypeNotFoundError {}
