use crate::model::{SafeString, Slot};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertUser {
    pub name: SafeString,
    pub slots: Vec<Slot>,
}
