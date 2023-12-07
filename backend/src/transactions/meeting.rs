use crate::{
    model::{DBMeeting, InsertMeeting, SafeString, TimeRange},
    routes::convert_err,
};
use sqlx::PgPool;

pub async fn insert_meeting(
    pool: &PgPool,
    insert_meeting: &InsertMeeting,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO meetings (id, name, range) VALUES ($1, $2, tstzrange($3, $4, '[]'))",
        id,
        insert_meeting.name.as_ref(),
        insert_meeting.range.start,
        insert_meeting.range.end
    )
    .execute(pool)
    .await?;

    Ok(id)
}

pub async fn select_meeting(pool: &PgPool, id: &uuid::Uuid) -> Result<DBMeeting, sqlx::Error> {
    match sqlx::query!("SELECT * FROM meetings WHERE id = $1", id)
        .fetch_optional(pool)
        .await?
    {
        None => {
            tracing::error!("No meeting found with id: {}", id);
            Err(sqlx::Error::RowNotFound)
        }
        Some(record) => Ok(DBMeeting {
            id: record.id,
            meeting: InsertMeeting {
                name: SafeString::parse(record.name).map_err(|_| {
                    convert_err("name", "Safe String contraint failed on name column.")
                })?,
                range: TimeRange::try_from((record.range.start, record.range.end)).map_err(
                    |_| convert_err("range", "Failed to parse range from the database."),
                )?,
            },
        }),
    }
}
