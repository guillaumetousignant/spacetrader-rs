use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RequestNotClonedError;

impl fmt::Display for RequestNotClonedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "request could not be cloned")
    }
}

impl error::Error for RequestNotClonedError {}
