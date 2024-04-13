use crate::data::{event::Event, user::User};
use askama::Template;
use uuid::Uuid;

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "parts/users_list.html")]
pub(crate) struct UsersList<'a, I>
where
    I: Iterator<Item = &'a User> + Clone,
{
    pub(crate) users: I,
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Template)]
#[template(path = "parts/events_list.html")]
pub(crate) struct EventsList<'a, 'b, I>
where
    I: Iterator<Item = &'a Event> + Clone,
{
    pub(crate) message: Option<&'b str>,

    pub(crate) user_uuid: Uuid,

    pub(crate) events: I,
}

///////////////////////////////////////////////////////////////////////////////////////////////

// #[derive(Template)]
// #[template(path = "events.html")]
// struct EventsTemplate {
//     events: Vec<Event>,
// }
