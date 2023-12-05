use crate::model::safe_string::SafeString;
use crate::model::timestamp::Timestamp24Hr;
use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertMeeting {
    pub name: SafeString,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub no_earlier_than: Timestamp24Hr,
    pub no_later_than: Timestamp24Hr,
}
