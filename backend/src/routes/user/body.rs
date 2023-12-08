use crate::model::{Availability, InsertUser, SafeString};

#[derive(serde::Deserialize)]
pub struct BodyData {
    pub name: String,
    pub availability: String,
}

impl TryFrom<BodyData> for InsertUser {
    type Error = String;

    fn try_from(body: BodyData) -> Result<Self, Self::Error> {
        Ok(Self {
            name: SafeString::parse(body.name.as_str())?,
            availability: Availability::try_from(body.availability.as_str())?,
        })
    }
}
