use chrono::{DateTime, Utc};
use non_empty_string::NonEmptyString;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct Event {
    name: NonEmptyString,
    date: DateTime<Utc>,
}
