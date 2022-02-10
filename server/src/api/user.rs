use futures::TryFutureExt;
use rocket::{http::CookieJar, serde::json::Json, State};
use std::fmt::Display;

use crate::{api::auth, api::ApiResponse, discord_client::DiscordClient, Env};
use theyrefor_models::User;

pub async fn get_current_user_id(token: impl Display, client: &State<DiscordClient>) -> ApiResponse<String> {
   client.get_current_user(token).await.map(|user| user.id)
}

#[get("/user")]
pub async fn get_user(env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>) -> Option<Json<User>> {
   auth::get_auth_token(env, cookies, client)
      .map_err(|_| ()) // deliberately swallow the redirect and just 404 instead
      .and_then(|token| client.get_current_user(token).map_err(|_| ()))
      .await
      .map(|user| match user.avatar {
         Some(_) => {
            let mut user = user;
            user.avatar = Some(format!(
               "https://cdn.discordapp.com/avatars/{}/{}.png",
               user.id.clone(),
               user.avatar.unwrap()
            ));
            Json(user)
         }
         None => Json(user),
      })
      .ok()
}
