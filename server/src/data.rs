use either::Either;
use log::error;
use reqwest::Client;
use rocket::{
   http::{CookieJar, Status},
   response::Redirect,
   serde::json::Value,
   State,
};

use crate::{auth::get_auth_token, Env};

#[get("/guilds")]
pub async fn get_guilds(
   cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>,
) -> Result<Value, Either<Status, Redirect>> {
   match get_auth_token(cookies, client, env).await {
      Err(redirect) => Err(Either::Right(redirect)),
      Ok(token) => {
         let client = reqwest::Client::new();
         let guilds = match client
            .get("https://discord.com/api/v8/users/@me/guilds")
            .bearer_auth(token)
            .send()
            .await
         {
            Ok(body) => body,
            Err(err) => {
               error!("{:?}", err);
               return Err(Either::Left(Status::InternalServerError));
            }
         };

         match guilds.text().await {
            Ok(data) => Ok(Value::String(data)),
            Err(err) => {
               error!("{:?}", err);
               Err(Either::Left(Status::InternalServerError))
            }
         }
      }
   }
}
