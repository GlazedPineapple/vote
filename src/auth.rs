use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
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
}

impl From<AccessTokenExchangeResponse> for OauthCookie {
    fn from(response: AccessTokenExchangeResponse) -> Self {
        let AccessTokenExchangeResponse {
            access_token,
            expires_in,
            refresh_token,
            token_type,
            ..
        } = response;

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();

        Self {
            access_token,
            expires_at: current_time + expires_in,
            refresh_token,
            token_type,
        }
    }
}
