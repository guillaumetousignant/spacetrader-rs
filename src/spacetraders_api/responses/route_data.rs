use super::LocationData;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RouteData {
    departure: LocationData,
    destination: LocationData,
    arrival: DateTime<Utc>,
    #[serde(rename = "departureTime")]
    departure_time: DateTime<Utc>,
}
