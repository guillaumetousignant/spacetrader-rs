use crate::spacetraders_api::System;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TraitNotFoundError {
    pub ship_symbol: String,
    pub trait_name: String,
    pub system: System,
}

impl fmt::Display for TraitNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} found no trait {} in system {}",
            self.ship_symbol, self.trait_name, self.system
        )
    }
}

impl error::Error for TraitNotFoundError {}
