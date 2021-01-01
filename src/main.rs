use rocket::{figment::{Figment, providers::Env}, launch, routes};
use serde::Deserialize;
use twilight_model::id::ApplicationId;
use twilight_oauth2::Client as OauthClient;

mod auth;
mod routes;

#[derive(Deserialize, Debug)]
pub struct Config {
    client_id: ApplicationId,
    client_secret: String,
    redirect_url: String
}

#[launch]
fn rocket() -> rocket::Rocket {
    dotenv::dotenv().ok();

    let config: Config = envy::from_env().expect("Missing required environment variables");

    let oauth = OauthClient::new(
        config.client_id,
        &config.client_secret,
        &[&config.redirect_url],
    ).expect("Failed to create oauth client");

    rocket::custom(Figment::from(rocket::Config::default()).merge(Env::prefixed("ROCKET_")))
        .manage(oauth)
        .manage(config)
        .mount("/", routes![routes::index, routes::auth::oauth_login])
}
