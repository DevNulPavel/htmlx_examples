use chrono::{DateTime, Utc};
use non_empty_string::NonEmptyString;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct Event {
    pub(crate) name: NonEmptyString,
    pub(crate) date: DateTime<Utc>,
}
