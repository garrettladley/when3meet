use crate::model::{
    fold, iso8601, DBMeeting, InsertMeeting, Meeting, SafeString, Slot, Timestamp24Hr, User,
};
use crate::routes::convert_err;
use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

#[tracing::instrument(
    name = "Reading a meeting given it's id.",
    skip(id, pool),
    fields(
        meeting_id = %id,
    )
)]
pub async fn read_meeting(id: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let id = match uuid::Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => {
            tracing::error!("Invalid meeting ID! Given: {}", id);
            return HttpResponse::BadRequest().json(format!("Invalid meeting ID! Given: {}", id));
        }
    };

    match select_meeting(&pool, &id).await {
        Ok(meeting) => HttpResponse::Ok().json(meeting),
        Err(e) => {
            tracing::error!("Failed to read meeting: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to read meeting.")
        }
    }
}

#[tracing::instrument(name = "Fetching meeting details from the database.", skip(id, pool))]
pub async fn select_meeting(pool: &PgPool, id: &uuid::Uuid) -> Result<Meeting, sqlx::Error> {
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

            let db_meeting = DBMeeting {
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
            };

            let users = sqlx::query!(
                r#"
                SELECT * FROM users
                WHERE meeting_id = $1
                "#,
                record.id
            )
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|record| {
                let slots: Vec<Slot> = fold(
                    record
                        .availability
                        .split('|')
                        .filter_map(|pair| {
                            let timestamps: Vec<&str> = pair.split('_').collect();
                            let start = match iso8601(timestamps[0]) {
                                Ok(start) => start.with_timezone(&Utc),
                                Err(_) => return None,
                            };

                            let end = match iso8601(timestamps[1]) {
                                Ok(end) => end.with_timezone(&Utc),
                                Err(_) => return None,
                            };

                            Some(Slot { start, end })
                        })
                        .collect(),
                );

                User {
                    id: record.id,
                    name: SafeString(record.name),
                    slots,
                }
            })
            .collect();

            Ok(Meeting {
                meeting: db_meeting,
                users,
            })
        }
    }
}
