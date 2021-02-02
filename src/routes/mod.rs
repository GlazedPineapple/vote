use std::{borrow::Cow, path::PathBuf};

use rocket::{
    get,
    http::{ContentType, Status},
    response::Content,
};
use rust_embed::RustEmbed;

use crate::{auth::AuthenticatedUser, templates::HelloTemplate};

pub mod auth;
pub mod polls;

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

#[get("/assets/<filename..>")]
pub fn assets(filename: PathBuf) -> Option<Content<Cow<'static, [u8]>>> {
    #[derive(RustEmbed)]
    #[folder = "assets/"]
    struct Assets;

    Assets::get(&filename.to_string_lossy()).map(|file| {
        Content(
            filename
                .extension()
                .map(|ex| ContentType::from_extension(&ex.to_string_lossy()))
                .flatten()
                .unwrap_or(ContentType::Binary),
            file,
        )
    })
}
