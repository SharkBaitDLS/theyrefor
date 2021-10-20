use either::Either;
use rocket::{
   http::{CookieJar, Status},
   response::Redirect,
   serde::json::Value,
};

use crate::auth::get_auth_token;

#[get("/guilds")]
pub async fn get_guilds(cookies: &CookieJar<'_>) -> Result<Value, Either<Status, Redirect>> {
   match get_auth_token(cookies) {
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
