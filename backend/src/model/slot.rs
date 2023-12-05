use crate::model::iso8601;
use chrono::{DateTime, Duration, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Slot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Slot {
    pub fn new(start: DateTime<Utc>) -> Self {
        Self {
            start,
            end: start + Duration::minutes(15),
        }
    }
}

impl TryFrom<&str> for Slot {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let start = match iso8601(value) {
            Ok(start) => start.with_timezone(&Utc),
            Err(_) => return Err("Invalid DateTime".to_string()),
        };

        Ok(Self::new(start))
    }
}

impl TryFrom<(&str, &str)> for Slot {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let start = match iso8601(value.0) {
            Ok(start) => start.with_timezone(&Utc),
            Err(_) => return Err("Invalid DateTime".to_string()),
        };

        let end = match iso8601(value.1) {
            Ok(end) => end.with_timezone(&Utc),
            Err(_) => return Err("Invalid DateTime".to_string()),
        };

        Ok(Self { start, end })
    }
}
