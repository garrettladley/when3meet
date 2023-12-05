use crate::model::{SafeString, Slot};
use cfg_if::cfg_if;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub name: SafeString,
    pub slots: Vec<Slot>,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::error::Error;
        use crate::model::time_strings::iso8601;
        use sqlx::{FromRow, sqlite::SqliteRow, Row};

        impl FromRow<'_, SqliteRow> for User {
            fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
                let construct_err = |name: &str, message: &str| {
                    sqlx::Error::ColumnDecode {
                        index: name.to_string(),
                        source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, message.to_string())),
                    }
                };

                let id = row.try_get("id")?;

                let name = row.try_get("name")?;
                let name =  match SafeString::parse(name) {
                    Ok(name) => name,
                    Err(_) => return Err(construct_err("name", "Invalid SafeString")),
                };

                let availability = row.try_get("availability")?;
                let availability: Result<Vec<Slot>, _> = availability.split('|').map(|slot| Slot::try_from(slot)).collect();
                let availability = match availability {
                    Ok(availability) => fold(availability),
                    Err(_) => return Err(construct_err("availability", "Invalid Availability")),
                };

               Ok(User {
                    id,
                    name,
                    slots: availability,
                })
            }
        }
    }
}
