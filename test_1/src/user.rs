use non_empty_string::NonEmptyString;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct User {
    uuid: Uuid,
    user_name: NonEmptyString,
    // name: NonEmptyString,
    // events: Vec<Event>,
}
