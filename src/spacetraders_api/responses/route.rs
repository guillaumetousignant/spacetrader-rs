use super::Location;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub departure: Location,
    pub destination: Location,
    pub arrival: DateTime<Utc>,
    #[serde(rename = "departureTime")]
    pub departure_time: DateTime<Utc>,
}
