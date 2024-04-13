use crate::data::{event::Event, user::User};
use askama::Template;

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
pub(crate) struct EventsList<'a, I>
where
    I: Iterator<Item = &'a Event> + Clone,
{
    pub(crate) events: I,
}

///////////////////////////////////////////////////////////////////////////////////////////////

// #[derive(Template)]
// #[template(path = "events.html")]
// struct EventsTemplate {
//     events: Vec<Event>,
// }
