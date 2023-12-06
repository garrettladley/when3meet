use crate::model::{iso8601, InsertMeeting, SafeString, Timestamp24Hr};
use actix_web::{web, HttpResponse};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct BodyData {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub no_earlier_than_hr: i32,
    pub no_earlier_than_min: i32,
    pub no_later_than_hr: i32,
    pub no_later_than_min: i32,
}

impl TryFrom<BodyData> for InsertMeeting {
    type Error = String;

    fn try_from(body: BodyData) -> Result<Self, Self::Error> {
        let name = SafeString::parse(body.name)?;
        let start = iso8601(&body.start_date)?;
        let end = iso8601(&body.end_date)?;
        let no_earlier_than =
            Timestamp24Hr::new(body.no_earlier_than_hr, body.no_earlier_than_min)?;
        let no_later_than = Timestamp24Hr::new(body.no_later_than_hr, body.no_later_than_min)?;

        Ok(Self {
            name,
            start,
            end,
            no_earlier_than,
            no_later_than,
        })
    }
}

#[tracing::instrument(
    name = "Creating a new meeting.",
    skip(body, pool),
    fields(
        meeting_name = %body.name,
        start_date = %body.start_date,
        end_date = %body.end_date,
        no_earlier_than_hr = %body.no_earlier_than_hr,
        no_earlier_than_min = %body.no_earlier_than_min,
        no_later_than_hr = %body.no_later_than_hr,
        no_later_than_min = %body.no_later_than_min,
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
