use log::error;
use reqwest::Client;
use rocket::{
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use serde::Deserialize;
use theyrefor_models::Guild;

use crate::{auth::get_auth_token, Env};

#[derive(Debug, Deserialize)]
struct DiscordGuild {
   pub name: String,
   pub id: String,
   pub icon: Option<String>,
}

impl From<DiscordGuild> for Guild {
   fn from(val: DiscordGuild) -> Self {
      Guild {
         name: val.name.clone(),
         id: val.id.parse().unwrap(),
         icon: val
            .icon
            .as_ref()
            .map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", val.id, icon)),
      }
   }
}

#[get("/guilds")]
pub async fn get_guilds(
   cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>,
) -> Result<Json<Vec<Guild>>, (Status, String)> {
   match get_auth_token(cookies, client, env).await {
      Err(redirect) => Err(redirect),
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
               return Err((Status::InternalServerError, String::new()));
            }
         };

         match guilds.json::<Vec<DiscordGuild>>().await {
            Ok(data) => Ok(Json(data.into_iter().map(|guild| guild.into()).collect())),
            Err(err) => {
               error!("{:?}", err);
               Err((Status::InternalServerError, String::new()))
            }
         }
      }
   }
}
