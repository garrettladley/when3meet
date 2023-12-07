use crate::model::safe_string::SafeString;
use crate::model::time_range::TimeRange;
#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertMeeting {
    pub name: SafeString,
    pub range: TimeRange,
}
