use askama::Template;
use rocket::{get, http::Status, response::content::Html};

pub mod auth;

#[get("/")]
pub fn index() -> Html<String> {
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
