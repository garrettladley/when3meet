use sqlx::PgPool;

use crate::{
    model::{availability, InsertUser, SafeString, User},
    routes::convert_err,
};

pub async fn select_user_by_meeting_id(
    pool: &PgPool,
    meeting_id: &uuid::Uuid,
) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query!(
        r#"
        SELECT * FROM users
        WHERE meeting_id = $1
        "#,
        meeting_id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|record| -> Result<User, sqlx::Error> {
        let user = User {
            id: record.id,
            user: InsertUser {
                name: SafeString::parse(record.name).map_err(|_| {
                    convert_err("name", "Safe String contraint failed on name column.")
                })?,
                slots: availability(&record.availability).map_err(|_| {
                    convert_err(
                        "availability",
                        "Slot formatting contraints failed on availability column.",
                    )
                })?,
            },
        };
        Ok(user)
    })
    .collect::<Result<Vec<_>, _>>()
}
