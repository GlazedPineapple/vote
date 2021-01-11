#![forbid(unsafe_code)]

use reqwest::Client as ReqClient;
use rocket::{
    figment::{providers::Env, Figment},
    launch, routes,
};
use serde::Deserialize;
use twilight_model::id::{ApplicationId, GuildId};
use twilight_oauth2::Client as OauthClient;

mod auth;
mod routes;
mod templates;

#[derive(Deserialize, Debug)]
pub struct Config {
    client_id: ApplicationId,
    client_secret: String,
    redirect_url: String,
    auth_cookie_domain: String,
    guild_id: GuildId,
}

#[launch]
fn rocket() -> rocket::Rocket {
    println!("your dad used unsafe and now you are here");

    dotenv::dotenv().ok();

    let config: Config = envy::from_env().expect("Missing required environment variables");

    let oauth = OauthClient::new(
        config.client_id,
        &config.client_secret,
        &[&config.redirect_url],
    )
    .expect("Failed to create oauth client");

    rocket::custom(Figment::from(rocket::Config::default()).merge(Env::prefixed("ROCKET_")))
        .manage(oauth)
        .manage(config)
        .manage(ReqClient::new())
        .mount(
            "/",
            routes![
                routes::index,
                routes::index_logged_in,
                routes::favicon,
                routes::auth::oauth_login,
                routes::auth::oauth_authorize
            ],
        )
}
