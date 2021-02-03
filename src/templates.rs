use std::collections::HashMap;

use askama::Template;
use chrono::{DateTime, Utc};
use twilight_model::id::UserId;
use uuid::Uuid;

use crate::models::{Candidates, PollRow};

macro_rules! derive_responder {
    ($($st:ty),+) => {
        use std::io::Cursor;
        use rocket::{
            http::ContentType,
            http::Status,
            response::{self, Responder},
            Request, Response,
        };

        $(
            impl<'r> Responder<'r, 'static> for $st {
                fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
                    let rsp = self.render().map_err(|_| Status::InternalServerError)?;
                    let content_type: ContentType = ContentType::from_extension(
                        self.extension().ok_or(Status::InternalServerError)?,
                    )
                    .ok_or(Status::InternalServerError)?;

                    Response::build()
                        .header(content_type)
                        .sized_body(rsp.len(), Cursor::new(rsp))
                        .ok()
                }
            }
        )*
    };
}

derive_responder!(HelloTemplate, HtmlRedirect<'r>, PollsList);

#[derive(Template, Debug)]
#[template(path = "hello.html")]
pub struct HelloTemplate {
    pub name: String,
    pub logged_in: bool,
}

/// Template for redirecting the user using html
///
/// This exists due to a quirk with web browsers not updating cookies until
/// after a redirect. By replacing the HTTP redirect with an html one, it lets
/// the browser save the cookie before navigating away
#[derive(Template, Debug)]
#[template(path = "redirect.html")]
pub struct HtmlRedirect<'a> {
    /// The url to navigate to
    pub url: &'a str,
}

impl<'a> HtmlRedirect<'a> {
    pub fn to(url: &'a str) -> Self {
        HtmlRedirect { url }
    }
}

#[derive(Template, Debug)]
#[template(path = "polls.html")]
pub struct PollsList {
    pub polls: Vec<PollRow>,
}

mod filters {
    use chrono::{DateTime, Utc};
    use humantime::format_duration;

    pub fn humantime<T: AsRef<DateTime<Utc>>>(time: T) -> askama::Result<String> {
        Ok(format_duration(
            Utc::now()
                .signed_duration_since(*time.as_ref())
                .to_std()
                .expect("Time went backwards"),
        )
        .to_string())
    }
}

