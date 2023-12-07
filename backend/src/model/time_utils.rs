use chrono::{DateTime, Utc};
use std::collections::Bound;

pub fn iso8601(raw: &str) -> Option<DateTime<Utc>> {
    match DateTime::parse_from_rfc3339(raw) {
        Ok(parsed) => Some(parsed.with_timezone(&Utc)),
        Err(_) => None,
    }
}

pub fn extract_from_bound(bound: std::collections::Bound<DateTime<Utc>>) -> Option<DateTime<Utc>> {
    match bound {
        Bound::Included(dt) | Bound::Excluded(dt) => Some(dt),
        std::collections::Bound::Unbounded => None,
    }
}
