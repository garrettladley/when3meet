use crate::model::db_meeting::DBMeeting;
use crate::model::user::User;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Meeting {
    pub meeting: DBMeeting,
    pub users: Vec<User>,
}
