pub fn convert_err(index: &str, message: &str) -> sqlx::Error {
    sqlx::Error::ColumnDecode {
        index: index.to_string(),
        source: Box::new(std::io::Error::new(std::io::ErrorKind::Other, message)),
    }
}
