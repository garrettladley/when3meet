use crate::model::{SafeString, Slot};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: SafeString,
    pub slots: Vec<Slot>,
}
