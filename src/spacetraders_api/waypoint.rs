use super::System;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Waypoint {
    pub sector: String,
    pub system: String,
    pub location: String,
}

impl Waypoint {
    pub fn system(&self) -> System {
        System {
            sector: self.sector.clone(),
            system: self.system.clone(),
        }
    }
}

impl fmt::Display for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.sector, self.system, self.location)
    }
}

impl PartialEq for Waypoint {
    fn eq(&self, other: &Self) -> bool {
        self.sector == other.sector
            && self.system == other.system
            && self.location == other.location
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseWaypointError;

impl FromStr for Waypoint {
    type Err = ParseWaypointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sector, system, location) = s
            .split_once('-')
            .and_then(|s1| match s1.1.split_once('-') {
                Some(s1_split) => Some((s1.0, s1_split.0, s1_split.1)),
                None => None,
            })
            .ok_or(ParseWaypointError)?;

        Ok(Waypoint {
            sector: sector.into(),
            system: system.into(),
            location: location.into(),
        })
    }
}

impl Serialize for Waypoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct WaypointVisitor;

impl<'de> Visitor<'de> for WaypointVisitor {
    type Value = Waypoint;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a waypoint string in the following format: X1-DF55-20250Z for sector, system and location.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Waypoint::from_str(value).or(Err(E::custom(format!(
            "str {} is not a valid waypoint",
            value
        ))))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Waypoint::from_str(&value).or(Err(E::custom(format!(
            "string {} is not a valid waypoint",
            value
        ))))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Waypoint::from_str(value).or(Err(E::custom(format!(
            "borrowed str {} is not a valid waypoint",
            value
        ))))
    }
}

impl<'de> Deserialize<'de> for Waypoint {
    fn deserialize<D>(deserializer: D) -> Result<Waypoint, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(WaypointVisitor)
    }
}
