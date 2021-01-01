use std::collections::HashSet;

use async_std::sync::RwLock;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use reqwest::header::HeaderMap;
use rocket::{get, http::Status, response::Redirect, uri, State};
use twilight_oauth2::{
    request::access_token_exchange::AccessTokenExchangeResponse, Client as OauthClient, Prompt,
    Scope,
};

use crate::Config;

const OAUTH_SCOPES: &[Scope] = &[Scope::Identify, Scope::Guilds];

lazy_static! {
    static ref STATES: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
}

#[get("/oauth/login")]
pub async fn oauth_login<'r>(oauth: State<OauthClient, 'r>, config: State<Config, 'r>) -> Redirect {
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

    STATES.write().await.insert(state);

    Redirect::to(auth_url)
}

#[get("/oauth/authorize?<code>&<state>")]
pub async fn oauth_authorize<'a, 'r>(
    oauth: State<'r, OauthClient>,
    config: State<'r, Config>,
    http: State<'r, reqwest::Client>,
    code: String,
    state: String,
) -> Result<Redirect, Status> {
    if !STATES.write().await.remove(&state) {
        return Err(Status::Forbidden);
    }

    let mut request = oauth
        .access_token_exchange(&code, &config.redirect_url)
        .expect("Redirect url is invalid");
    let request = request.scopes(OAUTH_SCOPES).build();

    let response: AccessTokenExchangeResponse = http
        .post(&request.url())
        .headers(
            request
                .headers
                .iter()
                .fold(HeaderMap::new(), |mut map, (header, value)| {
                    map.append(*header, value.parse().unwrap());
                    map
                }),
        )
        .form(&request.body)
        .send()
        .await
        .expect("Failed to make request")
        .error_for_status()
        .expect("Received an error from the server")
        .json()
        .await
        .expect("Failed to read response");

    dbg!(response);

    Ok(Redirect::to(uri!(super::index)))
}
