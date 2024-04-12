use crate::data::event::Event;
use non_empty_string::NonEmptyString;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct User {
    /// UUID юзера
    #[serde(with = "uuid::serde::braced")]
    pub(crate) uuid: Uuid,

    /// Имя юзера
    pub(crate) name: NonEmptyString,

    /// События юзера
    pub(crate) events: Vec<Event>,
}
