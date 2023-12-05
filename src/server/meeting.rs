use crate::model::{iso8601, meeting::Meeting, DBMeeting, SafeString, Timestamp24Hr};
use leptos::{server, ServerFnError};

#[server(GetMeeting, "/api")]
pub async fn get_meeting() -> Result<Meeting, ServerFnError> {
    use crate::model::DBMeeting;
    use crate::model::User;

    let mut conn = db().await?;

    let sql_query = r#"
        SELECT
            meetings.id AS meeting_id,
            meetings.name AS meeting_name,
            meetings.start_date AS meeting_start,
            meetings.end_date AS meeting_end,
            meetings.no_earlier_than_hr AS meeting_no_earlier_than_hr,
            meetings.no_earlier_than_min AS meeting_no_earlier_than_min,
            meetings.no_later_than_hr AS meeting_no_later_than_hr,
            meetings.no_later_than_min AS meeting_no_later_than_min,
            users.id AS user_id,
            users.name AS user_name,
            users.availability AS user_availability,
            users.meeting_id AS user_meeting_id
        FROM
            meetings
        JOIN
            users ON meetings.id = users.meeting_id;
    "#;

    let result = sqlx::query_as::<_, (DBMeeting, User)>(sql_query)
        .fetch_all(&mut conn)
        .await?;

    let meeting = result.into_iter().next();

    Ok(meeting)
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct RawMeeting {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub no_earlier_than_hr: u8,
    pub no_earlier_than_min: u8,
    pub no_later_than_hr: u8,
    pub no_later_than_min: u8,
}

impl TryInto<DBMeeting> for RawMeeting {
    type Error = ServerFnError;

    fn try_into(self) -> Result<DBMeeting, Self::Error> {
        let name = match SafeString::parse(self.name) {
            Ok(name) => name,
            Err(_) => return Err(ServerFnError::ServerError("Invalid SafeString".to_string())),
        };

        let start = match iso8601(&self.start_date) {
            Ok(start) => start,
            Err(_) => return Err(ServerFnError::ServerError("Invalid ISO8601".to_string())),
        };

        let end = match iso8601(&self.end_date) {
            Ok(end) => end,
            Err(_) => return Err(ServerFnError::ServerError("Invalid ISO8601".to_string())),
        };

        let net = match Timestamp24Hr::new(self.no_earlier_than_hr, self.no_earlier_than_min) {
            Ok(net) => net,
            Err(_) => {
                return Err(ServerFnError::ServerError(
                    "Invalid Timestamp24Hr".to_string(),
                ))
            }
        };

        let nlt = match Timestamp24Hr::new(self.no_later_than_hr, self.no_later_than_min) {
            Ok(nlt) => nlt,
            Err(_) => {
                return Err(ServerFnError::ServerError(
                    "Invalid Timestamp24Hr".to_string(),
                ))
            }
        };

        Ok(DBMeeting {
            id: None,
            name,
            start,
            end,
            no_earlier_than: net,
            no_later_than: nlt,
        })
    }
}

#[server(CreateMeeting, "/api")]
pub async fn create_meeting(raw_meeting: RawMeeting) -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    let meeting = raw_meeting.try_into()?;

    sqlx::query("INSERT INTO meeting (name, description, slots) VALUES ($1, $2, $3)")
        .bind(title)
        .bind(description)
        .bind(slots)
        .execute(&mut conn)
        .await?;

    Ok(())
}
