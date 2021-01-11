use rocket::{get, http::Status};

use crate::{auth::AuthenticatedUser, templates::HelloTemplate};

pub mod auth;

#[get("/")]
pub fn index_logged_in(user: AuthenticatedUser) -> HelloTemplate {
    dbg!(&user);

    HelloTemplate {
        name: user.name.clone(),
        logged_in: true,
    }
}

#[get("/", rank = 1)]
pub fn index() -> HelloTemplate {
    HelloTemplate {
        name: "cock".to_owned(),
        logged_in: false,
    }
}

#[get("/favicon.ico")]
pub fn favicon() -> Status {
    Status::NotFound
}
