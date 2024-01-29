// Can be removed when: https://github.com/SergioBenitez/Rocket/issues/2350 is closed
#![allow(clippy::let_unit_value)]
use base64::engine::{general_purpose::URL_SAFE, Engine};
use futures::TryFutureExt;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{
   http::{Cookie, CookieJar, SameSite, Status},
   response::Redirect,
   State,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

use super::{user, ApiError, ApiResponse};
use crate::{discord_client::DiscordClient, Env};
use theyrefor_models::AuthState;

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

fn build_auth_url(env: &State<Env>, cookies: &CookieJar<'_>) -> ApiError {
   let token: String = rand::thread_rng()
      .sample_iter(&Alphanumeric)
      .take(30)
      .map(char::from)
      .collect();

   let mut session_cookie = Cookie::new(SESSION_COOKIE_NAME, token.clone());
   // This has to be lax because we retrieve it when Discord redirects back to us for auth
   session_cookie.set_same_site(SameSite::Lax);
   session_cookie.set_path("/api/auth");
   if env.is_release {
      session_cookie.set_secure(true);
   }
   cookies.add_private(session_cookie);

   let state = AuthState {
      redirect_to: None,
      token,
   };
   (
      Status::Unauthorized,
      format!(
         "{}?client_id={}&redirect_uri={}&response_type=code&scope=identify%20guilds%20guilds.members.read&state={}",
         "https://discord.com/api/oauth2/authorize",
         env.client_id,
         urlencoding::encode(&format!("{}/api/auth", env.base_uri)),
         URL_SAFE.encode(bincode::serialize(&state).unwrap())
      ),
   )
}

pub async fn get_auth_token(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<String> {
   match cookies
      .get_private(TOKEN_COOKIE_NAME)
      .and_then(|cookie| serde_json::from_str::<AuthToken>(cookie.value()).ok())
   {
      Some(auth) if auth.expiration < Instant::now() => {
         refresh_token(auth, env, cookies, client).await.map(|auth| auth.token)
      }
      Some(auth) => Ok(auth.token),
      None => Err(build_auth_url(env, cookies)),
   }
}

fn set_auth_token(token: AuthToken, env: &State<Env>, cookies: &CookieJar<'_>) -> Result<AuthToken, serde_json::Error> {
   serde_json::to_string(&token).map(|serialized| {
      let mut auth_cookie = Cookie::new(TOKEN_COOKIE_NAME, serialized);
      auth_cookie.set_path("/api");
      if env.is_release {
         auth_cookie.set_secure(true);
      }
      cookies.add_private(auth_cookie);
      token
   })
}

async fn refresh_token(
   token: AuthToken, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<AuthToken> {
   update_token(
      DiscordRefreshRequest {
         client_id: &env.client_id,
         client_secret: &env.client_secret,
         grant_type: "refresh_token",
         refresh_token: &token.refresh_token,
      },
      env,
      cookies,
      client,
   )
   .await
   .map_err(|_| build_auth_url(env, cookies))
}

async fn update_token<T: Serialize>(
   request: T, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> Result<AuthToken, Status> {
   let token = client.update_token(request).await?;

   set_auth_token(
      AuthToken {
         token: token.access_token,
         expiration: Instant::now() + Duration::from_secs(token.expires_in),
         refresh_token: token.refresh_token,
      },
      env,
      cookies,
   )
   .map_err(|err| {
      error!("Could not serialize auth token: {:?}", err);
      Status::InternalServerError
   })
}

#[get("/login")]
pub async fn login(env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>) -> ApiResponse<()> {
   get_auth_token(env, cookies, client)
      // service call to force reauth in event of invalid token cookie
      .and_then(|token| user::get_current_user_id(token, client))
      .await
      .map(|_| ())
      .map_err(|_| build_auth_url(env, cookies))
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) {
   let token_cookie = Cookie::build(TOKEN_COOKIE_NAME).path("/api");
   cookies.remove_private(token_cookie);
}

#[get("/auth?<code>&<state>")]
pub async fn authorize(
   code: &str, state: &str, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> Result<Redirect, Status> {
   let state: AuthState = match URL_SAFE
      .decode(state)
      .ok()
      .as_ref()
      .and_then(|bytes| bincode::deserialize(bytes).ok())
   {
      Some(data) => data,
      None => return Err(Status::Forbidden),
   };
   if cookies
      .get_private(SESSION_COOKIE_NAME)
      .map(|cookie| cookie.value().to_owned())
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
      env,
      cookies,
      client,
   )
   .await
   .map(|_| Redirect::to(state.redirect_to.unwrap_or_else(|| "/".to_owned())))
}
