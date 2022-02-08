use futures::{FutureExt, TryFutureExt};
use reqwest_middleware::ClientWithMiddleware;
use rocket::{
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use theyrefor_models::User;

use crate::{auth::get_auth_token, util, Env};

pub async fn get_current_user_id(
   token: String, client: &State<ClientWithMiddleware>,
) -> Result<String, (Status, String)> {
   client
      .get("https://discord.com/api/v8/users/@me")
      .bearer_auth(token)
      .send()
      .then(util::deserialize::<User>)
      .await
      .map(|user| user.id)
}

#[get("/user")]
pub async fn get_user(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<ClientWithMiddleware>,
) -> Option<Json<User>> {
   get_auth_token(env, cookies, client)
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
