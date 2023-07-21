use crate::spacetraders_api::System;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TradeGoodNotFoundInSystemError {
    pub ship_symbol: String,
    pub trade_good: String,
    pub system: System,
}

impl fmt::Display for TradeGoodNotFoundInSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} found no trade good {} in system {}",
            self.ship_symbol, self.trade_good, self.system
        )
    }
}

impl error::Error for TradeGoodNotFoundInSystemError {}

#[derive(Debug, Clone)]
pub struct TradeGoodNotFoundInCargoError {
    pub ship_symbol: String,
    pub trade_good: String,
}

impl fmt::Display for TradeGoodNotFoundInCargoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} found no trade good {} in its cargo",
            self.ship_symbol, self.trade_good
        )
    }
}

impl error::Error for TradeGoodNotFoundInCargoError {}

#[derive(Debug, Clone)]
pub struct TradeGoodNotFoundInContractsError {
    pub ship_symbol: String,
    pub trade_good: String,
}

impl fmt::Display for TradeGoodNotFoundInContractsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Ship {} found no trade good {} in contracts",
            self.ship_symbol, self.trade_good
        )
    }
}

impl error::Error for TradeGoodNotFoundInContractsError {}
