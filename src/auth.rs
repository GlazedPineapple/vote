use std::time::{SystemTime, UNIX_EPOCH};

use rocket::futures::join;
use serde::{Deserialize, Serialize};
use twilight_http::Client;
use twilight_model::{id::GuildId, user::{CurrentUser, CurrentUserGuild}};
use twilight_oauth2::{request::access_token_exchange::AccessTokenExchangeResponse, TokenType};

#[derive(Debug, Deserialize, Serialize)]
pub struct OauthCookie {
    /// Access token to be used when making requests to the API on the user's
    /// behalf.
    pub access_token: String,
    /// The unix timestamp that this access token will expire.
    ///
    /// After this instant, the refresh token must be exchanged for another
    /// access token and refresh token pair.
    pub expires_at: u64,
    /// Refresh token to use to exchange for another access token and refresh
    /// token pair.
    pub refresh_token: String,
    /// Type of token provided.
    ///
    /// This will always be [`TokenType::Bearer`].
    ///
    /// [`TokenType::Bearer`]: ../enum.TokenType.html#variant.Bearer
    pub token_type: TokenType,
    /// The users info at the time of login
    pub user: CurrentUser,
}

pub enum OauthLoginError {
    NotInGuild,
}

impl OauthCookie {
    pub async fn login(response: AccessTokenExchangeResponse, guild_id: GuildId) -> Result<Self, OauthLoginError> {
        let AccessTokenExchangeResponse {
            access_token,
            expires_in,
            refresh_token,
            token_type,
            ..
        } = response;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let http = Client::new(format!("{} {}", token_type.name(), access_token));

        let (user, guilds) = join!(http.current_user(), http.current_user_guilds());

        let guilds: Vec<CurrentUserGuild> = guilds.expect("Failed to get the user's guilds");

        if guilds.iter().any(|x| x.id == guild_id) {
            Ok(Self {
                access_token,
                expires_at: current_time + expires_in,
                refresh_token,
                token_type,
                user: user.expect("Failed to get the user"),
            })
        } else {  
            Err(OauthLoginError::NotInGuild)
        }
    }
}
