use super::Location;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    departure: Location,
    destination: Location,
    arrival: DateTime<Utc>,
    #[serde(rename = "departureTime")]
    departure_time: DateTime<Utc>,
}
