use crate::data::user::User;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct Context {
    /// Используется дерево для постоянного порядка юзеров
    pub(crate) users: Mutex<BTreeMap<Uuid, User>>,
}
