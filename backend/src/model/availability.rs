use crate::model::Slot;
use chrono::SecondsFormat;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Availability(pub Vec<Slot>);

impl Availability {
    pub fn new(slots: Vec<Slot>) -> Self {
        Self(
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
                }),
        )
    }
}

impl TryFrom<&str> for Availability {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(
            match value
                .split('|')
                .map(|pair| {
                    let timestamps: Vec<&str> = pair.split('_').collect();
                    Slot::try_from((timestamps[0], timestamps[1]))
                })
                .collect::<Result<Vec<Slot>, String>>()
            {
                Ok(slots) => Availability::new(slots).0,
                Err(e) => return Err(e.to_string()),
            },
        ))
    }
}

impl std::fmt::Display for Availability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let binding = self.0.iter().fold(String::new(), |mut binding, slot| {
            binding.push_str(&format!(
                "{}_{}|",
                slot.start.to_rfc3339_opts(SecondsFormat::Secs, true),
                slot.end.to_rfc3339_opts(SecondsFormat::Secs, true)
            ));
            binding
        });

        let availability = binding.trim_end_matches('|');

        write!(f, "{}", availability)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{availability, Slot};
    use availability::Availability;
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

        let folded_slots = Availability::new(slots);

        assert_eq!(
            folded_slots.0,
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
        assert_eq!(Availability::new(vec![]).0, vec![]);
    }
}
