use crate::model::{SafeString, Slot};

pub struct User {
    id: i64,
    name: SafeString,
    slots: Vec<Slot>,
}
