use crate::model::{time_range::TimeRange, InsertMeeting, SafeString};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct BodyData {
    pub name: String,
    pub start: String,
    pub end: String,
}

impl TryFrom<BodyData> for InsertMeeting {
    type Error = String;

    fn try_from(body: BodyData) -> Result<Self, Self::Error> {
        Ok(Self {
            name: SafeString::parse(body.name)?,
            range: TimeRange::try_from((body.start.as_str(), body.end.as_str()))?,
        })
    }
}

#[tracing::instrument(
    name = "Creating a new meeting.",
    skip(body, pool),
    fields(
        meeting_name = %body.name,
        start = %body.start,
        end = %body.end
    )
)]
pub async fn create_meeting(body: web::Json<BodyData>, pool: web::Data<PgPool>) -> HttpResponse {
    let meeting = match InsertMeeting::try_from(body.into_inner()) {
        Ok(meeting) => meeting,
        Err(e) => {
            tracing::error!("Failed to parse meeting data: {:?}", e);
            return HttpResponse::BadRequest().finish();
        }
    };

    match insert_meeting(&pool, &meeting).await {
        Ok(response) => HttpResponse::Ok().body(response.to_string()),
        Err(e) => {
            tracing::error!("Failed to insert meeting into database: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Inserting new meeting details into the database.",
    skip(pool, meeting)
)]
pub async fn insert_meeting(
    pool: &PgPool,
    meeting: &InsertMeeting,
) -> Result<uuid::Uuid, sqlx::Error> {
    crate::transactions::insert_meeting(pool, meeting).await
}
