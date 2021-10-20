use reqwest::StatusCode;
use rocket::{
   http::{Cookie, CookieJar, SameSite, Status},
   response::Redirect,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

const TOKEN_COOKIE_NAME: &str = "token";

#[derive(Serialize, Deserialize)]
struct AuthToken {
   token: String,
   #[serde(with = "serde_millis")]
   expiration: Instant,
   refresh_token: String,
}

#[derive(Serialize)]
struct DiscordAuthRequest<'a> {
   client_id: &'a str,
   client_secret: &'a str,
   grant_type: &'a str,
   code: &'a str,
   redirect_uri: &'a str,
}

#[derive(Deserialize)]
struct DiscordAuthResponse {
   access_token: String,
   expires_in: u64,
   refresh_token: String,
}

pub fn get_auth_token(cookies: &CookieJar<'_>) -> Result<String, Redirect> {
   cookies
      .get_private(TOKEN_COOKIE_NAME)
      .and_then(|cookie| serde_json::from_str::<AuthToken>(cookie.value()).ok())
      .filter(|auth_token| auth_token.expiration > Instant::now())
      .map(|auth_token| auth_token.token)
      .ok_or_else(|| {
         Redirect::to(uri!(
            "https://discord.com/api/oauth2/authorize?client_id=lmao&\
          redirect_uri=http%3A%2F%2Flocalhost%3A8000%2Fapi%2Fauth&response_type=code&scope=guilds"
         ))
      })
}

fn set_auth_token(token: AuthToken, cookies: &CookieJar<'_>) -> Result<(), serde_json::Error> {
   serde_json::to_string(&token).map(|serialized| {
      let mut auth_cookie = Cookie::new("token", serialized);
      // TODO: remove once not testing via localhost/HTTP
      auth_cookie.set_secure(false);
      auth_cookie.set_same_site(SameSite::Lax);

      cookies.add_private(auth_cookie);
   })
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) {
   cookies.remove_private(Cookie::named(TOKEN_COOKIE_NAME));
}

#[get("/auth?<code>&<state>")]
pub async fn authorize(code: &str, state: Option<&str>, cookies: &CookieJar<'_>) -> Result<Redirect, Status> {
   let client = reqwest::Client::new();
   // TODO: MOVE TO ENVIRONMENT VARIABLES
   let body = match serde_urlencoded::to_string(&DiscordAuthRequest {
      client_id: "lmao",
      client_secret: "you thought",
      grant_type: "authorization_code",
      code,
      redirect_uri: "http://localhost:8000/api/auth",
   }) {
      Ok(json) => json,
      Err(_) => {
         error!("Malformed body could not be encoded into JSON");
         return Err(Status::new(500));
      }
   };

   let result = match client
      .post("https://discord.com/api/v8/oauth2/token")
      .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await
   {
      Ok(body) => body,
      Err(err) => {
         error!("Failed to send auth request: {:?}", err);
         return Err(Status::new(500));
      }
   };

   let token: DiscordAuthResponse = if result.status() == StatusCode::OK {
      match result.json().await {
         Ok(token) => token,
         Err(err) => {
            error!("Failed to decode auth request: {:?}", err);
            return Err(Status::new(500));
         }
      }
   } else {
      let status = result.status();
      error!("Auth request failed: {:?}", result.text().await);
      return Err(Status::new(status.into()));
   };

   if let Err(err) = set_auth_token(
      AuthToken {
         token: token.access_token,
         expiration: Instant::now() + Duration::from_secs(token.expires_in),
         refresh_token: token.refresh_token,
      },
      cookies,
   ) {
      error!("{:?}", err);
   }

   Ok(Redirect::to(uri!("/")))
}
