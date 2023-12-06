use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::model::{availability::Availability, InsertUser, SafeString};

#[derive(serde::Deserialize)]
pub struct BodyData {
    pub name: String,
    pub availability: String,
}

impl TryFrom<BodyData> for InsertUser {
    type Error = String;

    fn try_from(body: BodyData) -> Result<Self, Self::Error> {
        let name = SafeString::parse(body.name)?;
        let availability = Availability::try_from(body.availability.as_ref())?;

        Ok(Self { name, availability })
    }
}
#[tracing::instrument(
    name = "Inserting a user for the given meeting ID.",
    skip(meeting_id, body, pool),
    fields(
        meeting_id = %meeting_id,
        user_name = %body.name,
        availability = %body.availability,
    )
)]
pub async fn create_user(
    meeting_id: web::Path<String>,
    body: web::Json<BodyData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let meeting_id = match uuid::Uuid::parse_str(&meeting_id) {
        Ok(meeting_id) => meeting_id,
        Err(_) => {
            tracing::error!("Invalid meeting ID! Given: {}", meeting_id);
            return HttpResponse::BadRequest()
                .json(format!("Invalid meeting ID! Given: {}", meeting_id));
        }
    };

    let user = match InsertUser::try_from(body.into_inner()) {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to parse user data: {:?}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    match crate::transactions::user::insert_user(&pool, &meeting_id, &user).await {
        Ok(user_id) => HttpResponse::Ok().json(user_id),
        Err(e) => {
            tracing::error!("Failed to insert user: {}", e);
            HttpResponse::InternalServerError().json("Failed to insert user.")
        }
    }
}
