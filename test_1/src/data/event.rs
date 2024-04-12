use chrono::{DateTime, Utc};
use non_empty_string::NonEmptyString;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Event {
    pub(crate) name: NonEmptyString,

    #[serde(with = "chrono::serde::ts_nanoseconds")]
    pub(crate) date: DateTime<Utc>,
}
