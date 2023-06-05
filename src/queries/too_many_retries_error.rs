use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TooManyRetriesError;

impl fmt::Display for TooManyRetriesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "too many retries")
    }
}

impl error::Error for TooManyRetriesError {}
