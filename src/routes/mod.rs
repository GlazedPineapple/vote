use askama::Template;
use rocket::{get, response::content::Html};

pub mod auth;

#[get("/")]
pub fn index() -> Html<String> {
    Html(
        HelloTemplate { name: "cock" }
            .render()
            .expect("Failed to render template"),
    )
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str,
}
