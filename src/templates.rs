use askama::Template;

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

derive_responder!(HelloTemplate<'r>, HtmlRedirect<'r>);

#[derive(Template, Debug)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    pub name: &'a str,
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
