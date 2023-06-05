use reqwest::Response;
use std::time::Duration;

pub fn get_rate_limit(response: &Response) -> Result<Duration, Box<dyn std::error::Error>> {
    let limit = response.headers()["retry-after"].to_str()?;
    let limit = limit.parse::<u64>()?;
    Ok(Duration::from_secs(limit))
}
