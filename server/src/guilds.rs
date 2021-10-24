use futures::TryFutureExt;
use log::error;
use reqwest::Client;
use rocket::{
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use serde::Deserialize;
use theyrefor_models::Guild;

use crate::{
   auth::{get_auth_token, DiscordBotAuthBuilder},
   Env,
};

#[derive(Debug, Deserialize, PartialEq, Eq)]
struct DiscordGuild {
   pub name: String,
   pub id: String,
   pub icon: Option<String>,
}

impl From<DiscordGuild> for Guild {
   fn from(val: DiscordGuild) -> Self {
      Guild {
         name: val.name.clone(),
         id: val.id.clone(),
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
         let guilds: Vec<DiscordGuild> = match client
            .get("https://discord.com/api/v8/users/@me/guilds")
            .bearer_auth(token)
            .send()
            .and_then(|body| body.json())
            .await
         {
            Ok(data) => data,
            Err(err) => {
               error!("{:?}", err);
               return Err((Status::InternalServerError, String::new()));
            }
         };

         let bot_guilds: Vec<DiscordGuild> = match client
            .get("https://discord.com/api/v8/users/@me/guilds")
            .bot_auth(&env.bot_token)
            .send()
            .and_then(|body| body.json())
            .await
         {
            Ok(data) => data,
            Err(err) => {
               error!("{:?}", err);
               return Err((Status::InternalServerError, String::new()));
            }
         };

         Ok(Json(
            guilds
               .into_iter()
               .filter(|guild| bot_guilds.contains(guild))
               .map(|guild| guild.into())
               .collect(),
         ))
      }
   }
}
