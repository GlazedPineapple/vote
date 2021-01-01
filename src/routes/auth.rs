use rand::{thread_rng, Rng};
use rocket::{
    response::{Flash, Redirect},
    State,
};
use twilight_oauth2::{Client as OauthClient, Prompt, Scope};

use crate::Config;

const OAUTH_SCOPES: &[Scope] = &[Scope::Identify, Scope::Guilds];

#[rocket::get("/oauth/login")]
pub fn oauth_login(oauth: State<OauthClient>, config: State<Config>) -> Flash<Redirect> {
    let state = base64::encode_config(
        (0..32).map(|_| thread_rng().gen()).collect::<Vec<_>>(),
        base64::URL_SAFE_NO_PAD,
    );

    let auth_url = oauth
        .authorization_url(&config.redirect_url)
        .expect("Redirect url is invalid")
        .scopes(OAUTH_SCOPES)
        .prompt(Prompt::None)
        .state(&state)
        .build();

    Flash::new(Redirect::to(auth_url), "oauth_state", state)
}
