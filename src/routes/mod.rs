use askama::Template;
use rocket::{get, http::{CookieJar, Status}, response::content::Html};

use self::auth::OAUTH_COOKIE_NAME;

pub mod auth;

#[get("/")]
pub fn index(cookies: &CookieJar) -> Html<String> {
    let auth = cookies.get_private(OAUTH_COOKIE_NAME);

    dbg!(auth);

    Html(
        HelloTemplate { name: "cock" }
            .render()
            .expect("Failed to render template"),
    )
}

#[get("/favicon.ico")]
pub fn favicon() -> Status {
    Status::NotFound
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}
