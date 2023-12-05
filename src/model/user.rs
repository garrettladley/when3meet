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
        use sqlx::{FromRow, sqlite::SqliteRow, Row};
        use crate::model::fold;

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

                let availability: String = row.try_get("availability")?;
                let availability: Result<Vec<Slot>, String> = availability.split('|').map(|slot| Slot::try_from(slot)).collect();
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

        impl sqlx::Decode<'_, sqlx::sqlite::Sqlite> for User {
            fn decode(
                value: sqlx::sqlite::SqliteValueRef<'_>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let mut row = value.try_into_row()?;
                Ok(Self::from_row(&mut row)?)
            }
        }

        #[derive(sqlx::Type)]
        pub struct UserTuple(pub i64, pub String, pub String);

        impl sqlx::Type<sqlx::sqlite::Sqlite> for UserTuple {
            fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
                <Self as sqlx::Type<sqlx::sqlite::Sqlite>>::type_info()
            }
        }

    }
}
