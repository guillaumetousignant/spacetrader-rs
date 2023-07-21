use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidNextTargetError {
    pub ship_symbol: String,
}

impl fmt::Display for InvalidNextTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ship {} found no valid next target", self.ship_symbol)
    }
}

impl error::Error for InvalidNextTargetError {}
