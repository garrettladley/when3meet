use crate::{
    model::{DBMeeting, InsertMeeting, SafeString, Timestamp24Hr},
    routes::convert_err,
};
use sqlx::PgPool;

pub async fn insert_meeting(
    pool: &PgPool,
    insert_meeting: &InsertMeeting,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = uuid::Uuid::new_v4();

    sqlx::query!("INSERT INTO meetings (id, name, start_date, end_date, no_earlier_than_hr, no_earlier_than_min, no_later_than_hr, no_later_than_min) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)", 
    id,
    insert_meeting.name.as_ref(),
    insert_meeting.start,
    insert_meeting.end,
    insert_meeting.no_earlier_than.hr as i8,
    insert_meeting.no_earlier_than.min as i8,
    insert_meeting.no_later_than.hr as i8,
    insert_meeting.no_later_than.min as i8)
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
        Some(record) => {
            let no_earlier_than_hr = record.no_earlier_than_hr as i8;
            let no_earlier_than_min = record.no_earlier_than_min as i8;
            let no_later_than_hr = record.no_later_than_hr as i8;
            let no_later_than_min = record.no_later_than_min as i8;

            Ok(DBMeeting {
                id: record.id,
                meeting: InsertMeeting {
                    name: SafeString::parse(record.name).map_err(|_| {
                        convert_err("name", "Safe String contraint failed on name column.")
                    })?,
                    start: record.end_date,
                    end: record.end_date,
                    no_earlier_than: Timestamp24Hr::new(
                        no_earlier_than_hr as u8,
                        no_earlier_than_min as u8,
                    )
                    .map_err(|_| {
                        convert_err(
                            "no_earlier_than",
                            "Timestamp24Hr contraint failed on no_earlier_than column.",
                        )
                    })?,
                    no_later_than: Timestamp24Hr::new(
                        no_later_than_hr as u8,
                        no_later_than_min as u8,
                    )
                    .map_err(|_| {
                        convert_err(
                            "no_later_than",
                            "Timestamp24Hr contraint failed on no_later_than column.",
                        )
                    })?,
                },
            })
        }
    }
}
