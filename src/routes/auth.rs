use rocket::{State, response::Redirect};
use twilight_oauth2::Client as OauthClient;

#[rocket::get("/oauth/login?<from>")]
pub fn oauth_login(from: String, oauth: State<OauthClient>) -> Redirect {
    let auth_url = ();

    Redirect::to(/* auth_url */ "//google.com")
}
