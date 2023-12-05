use crate::model::dbmeeting::DBMeeting;
use crate::model::user::User;

pub struct Meeting {
    meeting: DBMeeting,
    users: Vec<User>,
}
