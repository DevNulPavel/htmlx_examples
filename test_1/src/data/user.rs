use crate::data::event::Event;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct User {
    pub(crate) uuid: Uuid,
    pub(crate) name: NonEmptyString,
    pub(crate) events: Vec<Event>,
    // pub(crate) name: NonEmptyString,
}
