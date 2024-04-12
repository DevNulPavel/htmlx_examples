use crate::data::event::Event;
use non_empty_string::NonEmptyString;
use std::collections::{BTreeMap, BTreeSet};
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
    pub(crate) events: BTreeMap<Uuid, Event>,
}

/*// Реализуем сравнения чисто только по UUID
impl PartialEq<User> for User {
    fn eq(&self, other: &User) -> bool {
        self.uuid.eq(&other.uuid)
    }
}

impl Eq for User {}

// Реализуем сравнения чисто только по UUID
impl PartialOrd<User> for User {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.uuid.partial_cmp(&other.uuid)
    }
}

// Реализуем сравнения чисто только по UUID
impl Ord for User {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
    }
}*/
