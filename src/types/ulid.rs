use serde::{Deserialize, Serialize};
use ulid::{Ulid};
use chrono::{DateTime, Utc};

#[derive(Clone, Eq)]
pub struct ULID(pub String);

impl<'de> Deserialize<'de> for ULID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        String::deserialize(deserializer)
            .map(ULID)
    }
}

impl ULID {
    pub fn timestamp(&self) -> DateTime<Utc> {
        Ulid::from_string(&self.0).unwrap().datetime()
    }
}

impl Serialize for ULID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        String::serialize(&self.0, serializer)
    }
}

impl std::ops::Deref for ULID {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::hash::Hash for ULID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl std::cmp::PartialEq for ULID {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl std::fmt::Display for ULID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::fmt::Debug for ULID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::str::FromStr for ULID {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}
