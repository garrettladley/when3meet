use crate::model::TimeRange;
use chrono::{DateTime, Utc};
use sqlx::postgres::types::PgRange;
use std::collections::Bound;

#[derive(serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Availability(pub Vec<TimeRange>);

impl Availability {
    pub fn new(slots: Vec<TimeRange>) -> Self {
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
        Ok(Availability::new(
            value
                .split('|')
                .map(|pair| {
                    let timestamps: Vec<&str> = pair.split('_').collect();
                    if timestamps.len() != 2 {
                        return Err("Invalid slot pair".to_string());
                    }
                    TimeRange::try_from((timestamps[0], timestamps[1]))
                })
                .collect::<Result<Vec<TimeRange>, String>>()?,
        ))
    }
}

impl TryFrom<Vec<(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)>> for Availability {
    type Error = String;

    fn try_from(
        value: Vec<(Bound<DateTime<Utc>>, Bound<DateTime<Utc>>)>,
    ) -> Result<Self, Self::Error> {
        Ok(Availability::new(
            value
                .into_iter()
                .map(TimeRange::try_from)
                .collect::<Result<Vec<TimeRange>, String>>()?,
        ))
    }
}

impl From<Availability> for Vec<PgRange<DateTime<Utc>>> {
    fn from(val: Availability) -> Self {
        val.0
            .into_iter()
            .map(|slot| PgRange {
                start: Bound::Included(slot.start),
                end: Bound::Excluded(slot.end),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{availability, TimeRange};
    use availability::Availability;
    use chrono::{DateTime, Utc};

    #[test]
    fn test_fold() {
        let slots = vec![
            TimeRange::fifteen(
                DateTime::parse_from_str("1693746000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            TimeRange::fifteen(
                DateTime::parse_from_str("1693746900", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            TimeRange::fifteen(
                DateTime::parse_from_str("1693748000", "%s")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
        ];

        let folded_slots = Availability::new(slots);

        assert_eq!(
            folded_slots.0,
            vec![
                TimeRange::new(
                    DateTime::parse_from_str("1693746000", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                    DateTime::parse_from_str("1693747800", "%s")
                        .unwrap()
                        .with_timezone(&Utc),
                )
                .unwrap(),
                TimeRange::fifteen(
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
