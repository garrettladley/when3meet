use crate::model::iso8601;
use crate::model::time_utils::extract_from_bound;
use chrono::{DateTime, Utc};
use std::collections::Bound;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, String> {
        if start > end {
            Err(format!(
                "Start time must be before end time: {} > {}",
                start, end
            ))
        } else {
            Ok(Self { start, end })
        }
    }

    pub fn fifteen(start: DateTime<Utc>) -> Self {
        Self {
            start,
            end: start + chrono::Duration::minutes(15),
        }
    }
}

impl TryFrom<&str> for TimeRange {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.split('|').collect::<Vec<&str>>();

        if value.len() != 2 {
            return Err(format!("Invalid time range: {}", value.join("-")));
        }

        let value = (value[0], value[1]);

        Self::try_from(value)
    }
}

impl TryFrom<(&str, &str)> for TimeRange {
    type Error = String;

    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        Self::new(
            iso8601(value.0)
                .ok_or_else(|| format!("Failed to parse start time from '{}'", value.0))?
                .with_timezone(&Utc),
            iso8601(value.1)
                .ok_or_else(|| format!("Failed to parse end time from '{}'", value.0))?
                .with_timezone(&Utc),
        )
    }
}

impl TryFrom<(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)> for TimeRange {
    type Error = String;

    fn try_from(value: (Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)) -> Result<Self, Self::Error> {
        Self::new(
            extract_from_bound(value.0)
                .ok_or_else(|| format!("Failed to parse start time from '{:?}'", value.0))?,
            extract_from_bound(value.1)
                .ok_or_else(|| format!("Failed to parse end time from '{:?}'", value.1))?,
        )
    }
}
