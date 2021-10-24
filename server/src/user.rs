use futures::TryFutureExt;
use reqwest::Client;
use rocket::{http::CookieJar, serde::json::Json, State};
use theyrefor_models::User;

use crate::{auth::get_auth_token, Env};

#[get("/user")]
pub async fn get_user(cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>) -> Option<Json<User>> {
   get_auth_token(cookies, client, env)
      .map_err(|_| ())
      .and_then(|token| {
         client
            .get("https://discord.com/api/v8/users/@me")
            .bearer_auth(token)
            .send()
            .map_err(|_| ())
      })
      .and_then(|res| res.json::<User>().map_err(|_| ()))
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
