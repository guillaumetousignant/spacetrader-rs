use reqwest::StatusCode;
use reqwest::Url;
use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct StatusError {
    pub status: StatusCode,
    pub message: String,
    pub url: Url,
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "received status {} with message {} from url {}",
            self.status, self.message, self.url
        )
    }
}

impl error::Error for StatusError {}
