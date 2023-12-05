use crate::model::InsertUser;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub user: InsertUser,
}
