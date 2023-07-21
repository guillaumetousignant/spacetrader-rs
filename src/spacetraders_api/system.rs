use super::Waypoint;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct System {
    pub sector: String,
    pub system: String,
}

impl System {
    pub fn waypoint(&self, location: String) -> Waypoint {
        Waypoint {
            sector: self.sector.clone(),
            system: self.system.clone(),
            location,
        }
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.sector, self.system)
    }
}

impl PartialEq for System {
    fn eq(&self, other: &Self) -> bool {
        self.sector == other.sector && self.system == other.system
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSystemError;

impl FromStr for System {
    type Err = ParseSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sector, system) = s.split_once('-').ok_or(ParseSystemError)?;

        Ok(System {
            sector: sector.into(),
            system: system.into(),
        })
    }
}

impl Serialize for System {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct SystemVisitor;

impl<'de> Visitor<'de> for SystemVisitor {
    type Value = System;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("a system string in the following format: X1-DF55 for sector and system.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        System::from_str(value).or(Err(E::custom(format!(
            "str {} is not a valid system",
            value
        ))))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        System::from_str(&value).or(Err(E::custom(format!(
            "string {} is not a valid system",
            value
        ))))
    }

    fn visit_borrowed_str<E>(self, value: &'de str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        System::from_str(value).or(Err(E::custom(format!(
            "borrowed str {} is not a valid system",
            value
        ))))
    }
}

impl<'de> Deserialize<'de> for System {
    fn deserialize<D>(deserializer: D) -> Result<System, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(SystemVisitor)
    }
}
