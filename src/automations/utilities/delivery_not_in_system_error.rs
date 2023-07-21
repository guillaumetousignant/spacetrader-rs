use crate::spacetraders_api::System;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct DeliveryNotInSystemError {
    pub ship_symbol: String,
    pub trade_good: String,
    pub ship_system: System,
    pub trade_good_system: System,
}

impl fmt::Display for DeliveryNotInSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} is in system {}, but delivery for trade good {} is in system {}",
            self.ship_symbol, self.ship_system, self.trade_good, self.trade_good_system
        )
    }
}

impl error::Error for DeliveryNotInSystemError {}
