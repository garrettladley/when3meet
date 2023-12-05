use chrono::{DateTime, Utc};

pub fn iso8601(raw: &str) -> Result<DateTime<Utc>, String> {
    let parsed = DateTime::parse_from_rfc3339(raw);
    match parsed {
        Ok(parsed) => Ok(parsed.with_timezone(&Utc)),
        Err(e) => Err(format!("Invalid date: {}", e)),
    }
}
