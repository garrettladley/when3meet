use crate::{
    model::{InsertUser, User},
    routes::user::BodyData,
};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[tracing::instrument(
    name = "Updating a user for the given user ID.",
    skip(user_id, body, pool),
    fields(
        user_id = %user_id,
        name = %body.name,
        availability = %body.availability,
    )
)]
pub async fn update_user(
    user_id: web::Path<String>,
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let user_id = match uuid::Uuid::parse_str(&user_id) {
        Ok(user_id) => user_id,
        Err(_) => {
            tracing::error!("Invalid user ID! Given: {}", user_id);
            return HttpResponse::BadRequest().json(format!("Invalid user ID! Given: {}", user_id));
        }
    };

    let user = match InsertUser::try_from(body.into_inner()) {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to parse user data: {:?}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    match update_user_db(&pool, &User { id: user_id, user }).await {
        Ok(user_id) => HttpResponse::Ok().body(user_id.to_string()),
        Err(e) => {
            tracing::error!("Failed to insert user: {}", e);
            HttpResponse::InternalServerError().json("Failed to insert user.")
        }
    }
}

#[tracing::instrument(name = "Updating a user for the given user ID.", skip(pool, user))]
pub async fn update_user_db(pool: &PgPool, user: &User) -> Result<uuid::Uuid, sqlx::Error> {
    crate::transactions::user::update_user(pool, user).await
}
