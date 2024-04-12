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
