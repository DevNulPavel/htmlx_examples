use chrono::{DateTime, Utc};
use non_empty_string::NonEmptyString;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Event {
    /// UUID события
    #[serde(with = "uuid::serde::braced")]
    pub(crate) uuid: Uuid,

    /// Имя события
    pub(crate) name: NonEmptyString,

    /// Дата события
    #[serde(with = "chrono::serde::ts_seconds")]
    pub(crate) date: DateTime<Utc>,
}

/* // Реализуем сравнения чисто только по UUID
impl PartialEq<Event> for Event {
    fn eq(&self, other: &Event) -> bool {
        self.uuid.eq(&other.uuid)
    }
}

impl Eq for Event {}

// Реализуем сравнения чисто только по UUID
impl PartialOrd<Event> for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}

// Реализуем сравнения чисто только по UUID
impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
} */
