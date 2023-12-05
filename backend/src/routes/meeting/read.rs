use crate::model::Meeting;
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[tracing::instrument(
    name = "Reading a meeting given it's id.",
    skip(meeting_id, pool),
    fields(
        meeting_id = %meeting_id,
    )
)]
pub async fn read_meeting(meeting_id: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let meeting_id = match uuid::Uuid::parse_str(&meeting_id) {
        Ok(meeting_id) => meeting_id,
        Err(_) => {
            tracing::error!("Invalid meeting ID! Given: {}", meeting_id);
            return HttpResponse::BadRequest()
                .json(format!("Invalid meeting ID! Given: {}", meeting_id));
        }
    };

    match select_meeting(&pool, &meeting_id).await {
        Ok(meeting) => HttpResponse::Ok().json(meeting),
        Err(e) => {
            tracing::error!("Failed to read meeting: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to read meeting.")
        }
    }
}

#[tracing::instrument(
    name = "Fetching meeting details from the database.",
    skip(meeting_id, pool)
)]
pub async fn select_meeting(
    pool: &PgPool,
    meeting_id: &uuid::Uuid,
) -> Result<Meeting, sqlx::Error> {
    match crate::transactions::select_meeting(pool, meeting_id).await {
        Err(e) => Err(e),
        Ok(db_meeting) => Ok(Meeting {
            meeting: db_meeting,
            users: crate::transactions::select_user_by_meeting_id(pool, meeting_id).await?,
        }),
    }
}
