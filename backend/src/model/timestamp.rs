#[derive(serde::Serialize, serde::Deserialize)]
pub struct Timestamp24Hr {
    pub hr: i32,
    pub min: i32,
}

impl Timestamp24Hr {
    pub fn new(hr: i32, min: i32) -> Result<Self, String> {
        if !(0..=23).contains(&hr) {
            Err(format!("Invalid hr: {}", hr))
        } else if !(0..=59).contains(&min) {
            Err(format!("Invalid min: {}", min))
        } else {
            Ok(Self { hr, min })
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
    fn test_invalid_hr() {
        let timestamp = Timestamp24Hr::new(24, 0);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid hr: 24".to_string()));

        let timestamp = Timestamp24Hr::new(25, 30);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid hr: 25".to_string()));
    }

    #[test]
    fn test_invalid_min() {
        let timestamp = Timestamp24Hr::new(12, 60);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid min: 60".to_string()));

        let timestamp = Timestamp24Hr::new(8, 61);
        assert!(timestamp.is_err());
        assert_eq!(timestamp.err(), Some("Invalid min: 61".to_string()));
    }
}
