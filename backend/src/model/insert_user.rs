use crate::model::{Availability, SafeString};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct InsertUser {
    pub name: SafeString,
    pub availability: Availability,
}
