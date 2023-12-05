use crate::model::{SafeString, Slot};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub name: SafeString,
    pub slots: Vec<Slot>,
}
