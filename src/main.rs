#![forbid(unsafe_code)]

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::io::{stdout, Write};

use reqwest::Client as ReqClient;
use rocket::{
    fairing::AdHoc,
    figment::{providers::Env, Figment},
    launch, routes,
};
use rocket_contrib::{database, helmet::SpaceHelmet};
use serde::Deserialize;
use twilight_model::id::{ApplicationId, GuildId};
use twilight_oauth2::Client as OauthClient;

mod auth;
mod models;
mod routes;
mod schema;
mod templates;

#[derive(Deserialize, Debug)]
pub struct Config {
    client_id: ApplicationId,
    client_secret: String,
    redirect_url: String,
    auth_cookie_domain: String,
    guild_id: GuildId,
}

#[database("polls")]
pub struct PollsDatabase(diesel::SqliteConnection);

#[launch]
fn rocket() -> rocket::Rocket {
    println!("your dad used unsafe and now you are here");

    let mut stdout = stdout();

    stdout.write_all(&[0x63, 0x6F, 0x63, 0x6B, 0x0A]).unwrap();

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
                routes::assets,
                routes::auth::oauth_login,
                routes::auth::oauth_authorize,
                routes::polls::all_polls,
                routes::polls::poll_by_id,
            ],
        )
        .attach(SpaceHelmet::default())
        .attach(PollsDatabase::fairing())
        .attach(AdHoc::on_attach("Database Migrations", |rocket| async {
            embed_migrations!();

            PollsDatabase::get_one(&rocket)
                .await
                .expect("Failed to create a db connection")
                .run(|conn| match embedded_migrations::run(&*conn) {
                    Ok(()) => Ok(rocket),
                    Err(e) => {
                        eprintln!("Failed to run database migrations: {:?}", e);
                        Err(rocket)
                    }
                })
                .await
        }))
}
