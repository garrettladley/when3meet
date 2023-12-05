#[derive(serde::Serialize, serde::Deserialize)]
pub struct Timestamp24Hr {
    pub hour: u8,
    pub minute: u8,
}
impl Timestamp24Hr {
    pub fn new(hour: u8, minute: u8) -> Result<Self, String> {
        if hour > 23 {
            Err(format!("Invalid hour: {}", hour))
        } else if minute > 59 {
            Err(format!("Invalid minute: {}", minute))
        } else {
            Ok(Self { hour, minute })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::timestamp::Timestamp24Hr;

    #[test]
    fn test_valid_timestamp() {
        let timestamp = Timestamp24Hr::new(12, 30);
        assert!(timestamp.is_ok());

        let timestamp = Timestamp24Hr::new(23, 59);
        assert!(timestamp.is_ok());
    }

    #[test]
    fn test_invalid_hour() {
        let timestamp = Timestamp24Hr::new(24, 0);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid hour: 24".to_string()));

        let timestamp = Timestamp24Hr::new(25, 30);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid hour: 25".to_string()));
    }

    #[test]
    fn test_invalid_minute() {
        let timestamp = Timestamp24Hr::new(12, 60);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid minute: 60".to_string()));

        let timestamp = Timestamp24Hr::new(8, 61);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid minute: 61".to_string()));
    }
}
