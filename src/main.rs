use rocket::{launch, routes};
use serde::Deserialize;
use twilight_model::id::ApplicationId;
use twilight_oauth2::Client as OauthClient;

mod auth;
mod routes;

#[derive(Deserialize, Debug)]
struct Config {
    client_id: ApplicationId,
    client_secret: String,
    // hostname: String
}

#[launch]
fn rocket() -> rocket::Rocket {
    dotenv::dotenv().ok();

    let config: Config = envy::from_env().unwrap();

    let oauth = OauthClient::new(
        config.client_id,
        config.client_secret,
        &["http://localhost:8080/oauth/authorize"],
    );

    rocket::ignite()
        .manage(oauth)
        .mount("/", routes![routes::index, routes::auth::oauth_login])
}
