use futures::TryFutureExt;
use reqwest::Client;
use rocket::{
   http::{Cookie, CookieJar, SameSite},
   serde::json::Json,
   State,
};
use theyrefor_models::User;

use crate::{auth::get_auth_token, Env};

const USER_COOKIE_NAME: &str = "user";

pub fn get_current_user_id(cookies: &CookieJar<'_>) -> Option<String> {
   cookies
      .get_private(USER_COOKIE_NAME)
      .map(|cookie| cookie.value().to_string())
}

fn set_current_user_id(id: String, env: &State<Env>, cookies: &CookieJar<'_>) {
   let mut user_cookie = Cookie::new(USER_COOKIE_NAME, id);
   // Allow testing on localhost without HTTPS
   if env.is_release {
      user_cookie.set_secure(true);
   } else {
      user_cookie.set_secure(false);
      user_cookie.set_same_site(SameSite::Lax);
   }

   cookies.add_private(user_cookie);
}

#[get("/user")]
pub async fn get_user(env: &State<Env>, cookies: &CookieJar<'_>, client: &State<Client>) -> Option<Json<User>> {
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
            set_current_user_id(user.id.clone(), env, cookies);
            Json(user)
         }
         None => Json(user),
      })
      .ok()
}
