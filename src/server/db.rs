use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Connection, SqliteConnection};
        use leptos::ServerFnError;

        pub async fn db() -> Result<SqliteConnection, ServerFnError> {
            let db_file_path = "./when3meet.db";

            if !std::path::Path::new(&db_file_path).exists() {
                std::fs::File::create(db_file_path)
                    .map_err(|err| {
                        ServerFnError::Args(format!("Failed to create database file: {}", err))
            })?;
            }

            Ok(SqliteConnection::connect(&format!("sqlite:{}",db_file_path)).await?)
        }
    }
}
