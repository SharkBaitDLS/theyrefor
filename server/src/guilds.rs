use std::{
   fs::File,
   io::{ErrorKind, Read},
   path::PathBuf,
};

use futures::{stream, StreamExt, TryFutureExt};
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
   user, Env,
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct DiscordGuild {
   pub name: String,
   pub id: String,
   pub icon: Option<String>,
   owner: bool,
}

#[derive(Debug, Deserialize)]
struct DiscordGuildRoles {
   pub roles: Vec<String>,
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

async fn get_mutual_guilds(
   token: String, env: &State<Env>, client: &State<Client>,
) -> Result<Vec<DiscordGuild>, (Status, String)> {
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

   Ok(guilds.into_iter().filter(|guild| bot_guilds.contains(guild)).collect())
}

#[get("/guilds")]
pub async fn get_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<Client>,
) -> Result<Json<Vec<Guild>>, (Status, String)> {
   match get_auth_token(env, cookies, client)
      .and_then(|token| get_mutual_guilds(token, env, client))
      .await
   {
      Err(redirect) => Err(redirect),
      Ok(guilds) => Ok(Json(guilds.into_iter().map(|guild| guild.into()).collect())),
   }
}

#[get("/guilds/admin")]
pub async fn get_admin_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<Client>,
) -> Result<Json<Vec<Guild>>, (Status, String)> {
   match get_auth_token(env, cookies, client)
      .and_then(|token| get_mutual_guilds(token, env, client))
      .await
   {
      Err(redirect) => Err(redirect),
      Ok(guilds) => {
         let admin_guilds = stream::iter(guilds)
            .filter_map(|guild| async move {
               if guild.owner {
                  Some(guild)
               } else {
                  match client
                     .get(format!(
                        "https://discord.com/api/v8/guilds/{}/members/{}",
                        guild.id,
                        user::get_current_user_id(cookies).unwrap() // a user has to be logged in at this point
                     ))
                     .bot_auth(&env.bot_token)
                     .send()
                     .and_then(|body| body.json::<DiscordGuildRoles>())
                     .await
                  {
                     Ok(data) => {
                        let path: PathBuf = [&env.clip_directory, &guild.id.to_string(), ".role_id"]
                           .iter()
                           .collect();

                        let mut admin_role_data = String::new();
                        if let Err(err) = File::open(&path).map(|mut file| file.read_to_string(&mut admin_role_data)) {
                           if err.kind() != ErrorKind::NotFound {
                              error!("Could not retrieve role ID for guild {:?}: {:?}", guild.id, err);
                           }
                        }

                        if data.roles.contains(&admin_role_data) {
                           Some(guild)
                        } else {
                           None
                        }
                     }
                     Err(err) => {
                        error!("Could not retrieve guild roles for user: {:?}", err);
                        None
                     }
                  }
               }
            })
            .collect::<Vec<_>>()
            .await;

         Ok(Json(admin_guilds.into_iter().map(|guild| guild.into()).collect()))
      }
   }
}
