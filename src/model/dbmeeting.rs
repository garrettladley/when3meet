use crate::model::safe_string::SafeString;
use crate::model::timestamp::Timestamp24Hr;
use cfg_if::cfg_if;
use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DBMeeting {
    pub id: Option<i64>,
    pub name: SafeString,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub no_earlier_than: Timestamp24Hr,
    pub no_later_than: Timestamp24Hr,
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::model::time_strings::iso8601;
        use sqlx::{FromRow, sqlite::SqliteRow, Row};

        impl FromRow<'_, SqliteRow> for DBMeeting {
            fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
                let construct_err = |name: &str, message: &str| {
                    sqlx::Error::ColumnDecode {
                        index: name.to_string(),
                        source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, message.to_string())),
                    }
                };

                let name = row.try_get("name")?;
                let name =  match SafeString::parse(name) {
                    Ok(name) => name,
                    Err(_) => return Err(construct_err("name", "Invalid SafeString")),
                };

                let start = row.try_get("start")?;
                let start = match iso8601(start) {
                    Ok(start) => start,
                    Err(_) => return Err(construct_err("start", "Invalid ISO8601")),
                };


                let end = row.try_get("end")?;
                let end = match iso8601(end) {
                    Ok(end) => end,
                    Err(_) => return Err(construct_err("end", "Invalid ISO8601")),
                };


                let net_hr = row.try_get("no_earlier_than_hr")?;
                let net_min = row.try_get("no_earlier_than_min")?;
                let net = match Timestamp24Hr::new(net_hr, net_min) {
                    Ok(net) => net,
                    Err(_) => return Err(construct_err("no_earlier_than_hr", "Invalid Timestamp24Hr")),
                };

                let nlt_hr = row.try_get("no_later_than_hr")?;
                let nlt_min = row.try_get("no_later_than_min")?;
                let nlt = match Timestamp24Hr::new(nlt_hr, nlt_min) {
                    Ok(nlt) => nlt,
                    Err(_) => return Err(construct_err("no_later_than_hr", "Invalid Timestamp24Hr")),
                };

                Ok(DBMeeting {
                    id: row.try_get("id")?,
                    name,
                    start,
                    end,
                    no_earlier_than: net,
                    no_later_than: nlt,
                })
            }
        }
    }
}
