use crate::data::user::User;
use parking_lot::Mutex;

///////////////////////////////////////////////////////////////////////////////////////////////

pub(crate) struct Context {
    pub(crate) users: Mutex<Vec<User>>,
}