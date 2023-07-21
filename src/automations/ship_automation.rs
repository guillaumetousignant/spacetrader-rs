use serde::{Deserialize, Serialize};
use std::error;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ShipAutomation {
    Command,
    Mining,
    Probe,
    None,
}

#[derive(Debug, Clone)]
pub struct UnknownShipAutomationError {
    pub value: String,
}

impl fmt::Display for UnknownShipAutomationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unknown ship automation: \"{}\"", self.value)
    }
}

impl error::Error for UnknownShipAutomationError {}
