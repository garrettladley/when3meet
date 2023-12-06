use crate::{
    model::{Availability, InsertUser, SafeString, User},
    routes::convert_err,
};
use sqlx::PgPool;

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
                availability: Availability::try_from(record.availability.as_ref()).map_err(
                    |e| {
                        convert_err(
                            "availability",
                            format!(
                                "Availability contraint failed on availability column. {}",
                                e
                            )
                            .as_str(),
                        )
                    },
                )?,
            },
        };
        Ok(user)
    })
    .collect::<Result<Vec<_>, _>>()
}

pub async fn insert_user(
    pool: &PgPool,
    meeting_id: &uuid::Uuid,
    user: &InsertUser,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO users (id, name, availability, meeting_id)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        user.name.as_ref(),
        user.availability.to_string(),
        meeting_id,
    )
    .execute(pool)
    .await?;

    Ok(id)
}

pub async fn update_user(pool: &PgPool, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET name = $1, availability = $2
        WHERE id = $3
        "#,
        user.user.name.as_ref(),
        user.user.availability.to_string(),
        user.id,
    )
    .execute(pool)
    .await?;

    Ok(())
}
