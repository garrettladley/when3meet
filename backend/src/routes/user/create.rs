use actix_web::{web, HttpResponse};
use sqlx::PgPool;

use crate::model::{availability, InsertUser, SafeString};

#[derive(serde::Deserialize)]
pub struct BodyData {
    pub name: String,
    pub availability: String,
}

impl TryFrom<BodyData> for InsertUser {
    type Error = String;

    fn try_from(body: BodyData) -> Result<Self, Self::Error> {
        let name = SafeString::parse(body.name)?;
        let availability = availability(&body.availability)?;

        Ok(Self {
            name,
            slots: availability,
        })
    }
}
#[tracing::instrument(
    name = "Reading a user for the given meeting id.",
    skip(meeting_id, pool),
    fields(
        id = %meeting_id,
    )
)]
pub async fn create_user(meeting_id: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let meeting_id = match uuid::Uuid::parse_str(&meeting_id) {
        Ok(meeting_id) => meeting_id,
        Err(_) => {
            tracing::error!("Invalid meeting ID! Given: {}", meeting_id);
            return HttpResponse::BadRequest()
                .json(format!("Invalid meeting ID! Given: {}", meeting_id));
        }
    };

    HttpResponse::Ok().json("Hello, world!")
}
