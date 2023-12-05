use crate::model::safe_string::SafeString;
use cfg_if::cfg_if;
use chrono::{DateTime, Utc};

pub struct DBMeeting {
    id: i64,
    name: SafeString,
    no_earlier_than: DateTime<Utc>,
    no_later_than: DateTime<Utc>,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::sqlite::SqliteRow;
        use sqlx::Row;
        use std::error::Error;

        impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for DBMeeting {
            fn from_row(row: &'_ sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
                let name: String = row.try_get("name")?;
                let name = SafeString::parse(name).map_err(|error| {
                    sqlx::Error::Decode(Box::new(sqlx::error::ColumnDecode {
                        index: "name",
                        source: Box::new(error) as Box<dyn Error + Send + Sync>,
                    }))
                })?;

                Ok(DBMeeting {
                    id: row.try_get("id")?,
                    name,
                    no_earlier_than: row.try_get("no_earlier_than")?,
                    no_later_than: row.try_get("no_later_than")?,
                })
            }
        }
    }
}
