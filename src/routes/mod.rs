use rocket::{
    get,
    http::{CookieJar, Status},
};

use crate::templates::HelloTemplate;

use self::auth::OAUTH_COOKIE_NAME;

pub mod auth;

#[get("/")]
pub fn index(cookies: &CookieJar) -> HelloTemplate<'static> {
    let auth = cookies.get_private(OAUTH_COOKIE_NAME);

    dbg!(&auth);

    HelloTemplate {
        name: "cock",
        logged_in: auth.is_some(),
    }
}

#[get("/favicon.ico")]
pub fn favicon() -> Status {
    Status::NotFound
}
