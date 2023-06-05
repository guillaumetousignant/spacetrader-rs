use reqwest::StatusCode;
use reqwest::Url;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StatusError {
    pub status: StatusCode,
    pub url: Url,
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "received status {} from url {}", self.status, self.url)
    }
}

impl error::Error for StatusError {}
