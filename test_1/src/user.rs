use crate::event::Event;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct User {
    pub(crate) uuid: Uuid,
    pub(crate) user_name: NonEmptyString,
    pub(crate) name: NonEmptyString,
    pub(crate) events: Vec<Event>,
}
