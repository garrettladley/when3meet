use crate::model::iso8601;
use chrono::{DateTime, Duration, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Slot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl Slot {
    pub fn new(start: DateTime<Utc>) -> Self {
        Self {
            start,
            end: start + Duration::minutes(15),
        }
    }
}

impl TryFrom<&str> for Slot {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let start = match iso8601(value) {
            Ok(start) => start.with_timezone(&Utc),
            Err(_) => return Err("Invalid DateTime".to_string()),
        };

        Ok(Self::new(start))
    }
}

pub fn fold(slots: Vec<Slot>) -> Vec<Slot> {
    slots
        .into_iter()
        .fold(Vec::new(), |mut folded_slots, slot| {
            if let Some(last_slot) = folded_slots.last_mut() {
                if last_slot.end == slot.start {
                    last_slot.end = slot.end;
                    return folded_slots;
                }
            }
            folded_slots.push(slot);
            folded_slots
        })
}

#[cfg(test)]
mod tests {
    use crate::model::{fold, Slot};
    use chrono::{DateTime, Utc};

    #[test]
    fn test_fold() {
        let slots = vec![
            Slot::new(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            Slot::new(
                DateTime::parse_from_str("1693746900", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            Slot::new(
                DateTime::parse_from_str("1693748000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        ];

        let folded_slots = fold(slots);

        assert_eq!(
            folded_slots,
            vec![
                Slot {
                    start: DateTime::parse_from_str("1693746000", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    end: DateTime::parse_from_str("1693747800", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                },
                Slot::new(
                    DateTime::parse_from_str("1693748000", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                ),
            ]
        );
    }

    #[test]
    fn test_fold_empty() {
        assert_eq!(fold(vec![]), vec![]);
    }
}
