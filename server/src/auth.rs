use rand::{distributions::Alphanumeric, Rng};
use reqwest::{Client, RequestBuilder, StatusCode};
use rocket::{
   http::{Cookie, CookieJar, SameSite, Status},
   response::Redirect,
   State,
};
use serde::{Deserialize, Serialize};
use std::{
   fmt,
   time::{Duration, Instant},
};
use theyrefor_models::AuthState;

use crate::Env;

const TOKEN_COOKIE_NAME: &str = "token";
const SESSION_COOKIE_NAME: &str = "session";

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

#[derive(Serialize)]
struct DiscordRefreshRequest<'a> {
   client_id: &'a str,
   client_secret: &'a str,
   grant_type: &'a str,
   refresh_token: &'a str,
}

#[derive(Deserialize)]
struct DiscordAuthResponse {
   access_token: String,
   expires_in: u64,
   refresh_token: String,
}

pub(crate) trait DiscordBotAuthBuilder {
   fn bot_auth<T: fmt::Display>(self, token: T) -> RequestBuilder;
}

impl DiscordBotAuthBuilder for RequestBuilder {
   fn bot_auth<T>(self, token: T) -> RequestBuilder
   where
      T: fmt::Display,
   {
      let header_value = format!("Bot {}", token);
      self.header(reqwest::header::AUTHORIZATION, header_value)
   }
}

fn build_auth_url(env: &State<Env>, cookies: &CookieJar<'_>) -> (Status, String) {
   let token: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(30)
      .map(char::from)
      .collect();

   let mut session_cookie = Cookie::new(SESSION_COOKIE_NAME, token.clone());
   // TODO: remove once not testing via localhost/HTTP
   session_cookie.set_secure(false);
   session_cookie.set_same_site(SameSite::Lax);
   cookies.add_private(session_cookie);

   let state = AuthState {
      redirect_to: None,
      token,
   };
   (
      Status::Unauthorized,
      format!(
         "{}?client_id={}&redirect_uri={}&response_type=code&scope=guilds&state={}",
         "https://discord.com/api/oauth2/authorize",
         env.client_id,
         urlencoding::encode(&format!("{}/api/auth", env.base_uri)),
         base64::encode(bincode::serialize(&state).unwrap())
      ),
   )
}

pub async fn get_auth_token(
   cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>,
) -> Result<String, (Status, String)> {
   match cookies
      .get_private(TOKEN_COOKIE_NAME)
      .and_then(|cookie| serde_json::from_str::<AuthToken>(cookie.value()).ok())
   {
      Some(auth) if auth.expiration > Instant::now() => {
         refresh_token(auth, cookies, client, env).await.map(|auth| auth.token)
      }
      Some(auth) => Ok(auth.token),
      None => Err(build_auth_url(env, cookies)),
   }
}

fn set_auth_token(token: AuthToken, cookies: &CookieJar<'_>) -> Result<AuthToken, serde_json::Error> {
   serde_json::to_string(&token).map(|serialized| {
      let mut auth_cookie = Cookie::new(TOKEN_COOKIE_NAME, serialized);
      // TODO: remove once not testing via localhost/HTTP
      auth_cookie.set_secure(false);
      auth_cookie.set_same_site(SameSite::Lax);

      cookies.add_private(auth_cookie);
      token
   })
}

async fn refresh_token(
   token: AuthToken, cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>,
) -> Result<AuthToken, (Status, String)> {
   update_token(
      DiscordRefreshRequest {
         client_id: &env.client_id,
         client_secret: &env.client_secret,
         grant_type: "refresh_token",
         refresh_token: &token.refresh_token,
      },
      cookies,
      client,
   )
   .await
   .map_err(|_| build_auth_url(env, cookies))
}

async fn update_token<T: Serialize>(
   request: T, cookies: &CookieJar<'_>, client: &State<Client>,
) -> Result<AuthToken, Status> {
   let body = match serde_urlencoded::to_string(request) {
      Ok(encoded) => encoded,
      Err(_) => {
         error!("Malformed body could not be encoded");
         return Err(Status::InternalServerError);
      }
   };

   let result = client
      .post("https://discord.com/api/v8/oauth2/token")
      .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
      .body(body)
      .send()
      .await;

   match result {
      Err(err) => {
         error!("Failed to send auth request: {:?}", err);
         Err(Status::InternalServerError)
      }
      Ok(response) => {
         let token: DiscordAuthResponse = if response.status() == StatusCode::OK {
            match response.json().await {
               Ok(token) => token,
               Err(err) => {
                  error!("Failed to decode auth request: {:?}", err);
                  return Err(Status::InternalServerError);
               }
            }
         } else {
            let status = response.status();
            error!("Auth request failed: {:?}", response.text().await);
            return Err(Status::new(status.into()));
         };

         set_auth_token(
            AuthToken {
               token: token.access_token,
               expiration: Instant::now() + Duration::from_secs(token.expires_in),
               refresh_token: token.refresh_token,
            },
            cookies,
         )
         .map_err(|err| {
            error!("{:?}", err);
            Status::InternalServerError
         })
      }
   }
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) {
   cookies.remove_private(Cookie::named(TOKEN_COOKIE_NAME));
}

#[get("/auth?<code>&<state>")]
pub async fn authorize(
   code: &str, state: &str, cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>,
) -> Result<Redirect, Status> {
   let state: AuthState = match base64::decode(state)
      .ok()
      .as_ref()
      .and_then(|bytes| bincode::deserialize(bytes).ok())
   {
      Some(data) => data,
      None => return Err(Status::Forbidden),
   };
   if cookies
      .get_private(SESSION_COOKIE_NAME)
      .map(|cookie| cookie.value().to_string())
      .filter(|session| *session == state.token)
      .is_none()
   {
      return Err(Status::Forbidden);
   }

   update_token(
      DiscordAuthRequest {
         client_id: &env.client_id,
         client_secret: &env.client_secret,
         grant_type: "authorization_code",
         code,
         redirect_uri: &format!("{}/api/auth", env.base_uri),
      },
      cookies,
      client,
   )
   .await
   .map(|_| Redirect::to(state.redirect_to.unwrap_or_else(|| "/".to_string())))
}
