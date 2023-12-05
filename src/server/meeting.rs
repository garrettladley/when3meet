use crate::model::meeting::Meeting;
use leptos::{server, ServerFnError};

#[server(GetMeeting, "/api")]
pub async fn get_meeting() -> Result<Meeting, ServerFnError> {
    use crate::model::DBMeeting;
    let mut conn = db().await?;

    sqlx::query_as::<_, DBmeeting>("SELECT * FROM meeting")
        .fetch_one(&mut conn)?
        .into()
}

#[server(CreateMeeting, "/api")]
pub async fn create_meeting() -> Result<(), ServerFnError> {
    let mut conn = db().await?;

    let no_earlier_than = iso8601(&meeting.no_earlier_than);
    let no_later_than = iso8601(&meeting.no_later_than);

    sqlx::query("INSERT INTO meeting (name, description, slots) VALUES ($1, $2, $3)")
        .bind(title)
        .bind(description)
        .bind(slots)
        .execute(&mut conn)
        .await?;

    Ok(())
}
