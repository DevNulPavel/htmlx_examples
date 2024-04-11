use chrono::{DateTime, Utc};
use non_empty_string::NonEmptyString;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub(crate) struct Event {
    pub(crate) name: NonEmptyString,
    pub(crate) date: DateTime<Utc>,
}
