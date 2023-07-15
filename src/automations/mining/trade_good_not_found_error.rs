use crate::spacetraders_api::System;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TradeGoodNotFoundError {
    pub ship_symbol: String,
    pub trade_good: String,
    pub system: System,
}

impl fmt::Display for TradeGoodNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} found no trade good {} in system {}",
            self.ship_symbol, self.trade_good, self.system
        )
    }
}

impl error::Error for TradeGoodNotFoundError {}
