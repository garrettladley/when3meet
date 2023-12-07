use crate::{
    model::{Availability, InsertUser, SafeString, User},
    routes::convert_err,
};
use chrono::{DateTime, Utc};
use sqlx::{postgres::types::PgRange, PgPool};
use std::collections::Bound;

pub async fn select_user_by_meeting_id(
    pool: &PgPool,
    meeting_id: &uuid::Uuid,
) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query!(
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
                    convert_err("name", "Safe String constraint failed on the name column.")
                })?,
                availability: Availability::try_from(
                    record
                        .availability
                        .iter()
                        .map(|pair| (pair.start, pair.end))
                        .collect::<Vec<(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)>>(),
                )
                .map_err(|_| {
                    convert_err(
                        "availability",
                        "Failed to parse availability from the database.",
                    )
                })?,
            },
        };

        Ok(user)
    })
    .collect::<Result<Vec<_>, _>>()?;

    Ok(users)
}

pub async fn insert_user(
    pool: &PgPool,
    meeting_id: &uuid::Uuid,
    user: InsertUser,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    let availability: Vec<PgRange<DateTime<Utc>>> = user.availability.into();

    sqlx::query!(
        r#"
        INSERT INTO users (id, name, availability, meeting_id)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        user.name.as_ref(),
        availability,
        meeting_id,
    )
    .execute(pool)
    .await?;

    Ok(id)
}

pub async fn update_user(pool: &PgPool, user: User) -> Result<uuid::Uuid, sqlx::Error> {
    let availability: Vec<PgRange<DateTime<Utc>>> = user.user.availability.into();

    sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, availability = $2
        WHERE id = $3
        "#,
        user.user.name.as_ref(),
        availability,
        user.id,
    )
    .execute(pool)
    .await?;

    Ok(user.id)
}
