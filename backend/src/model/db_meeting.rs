use crate::model::InsertMeeting;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DBMeeting {
    pub id: Uuid,
    pub meeting: InsertMeeting,
}
