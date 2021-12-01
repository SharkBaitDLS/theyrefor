use std::{
   fs::File,
   io::{ErrorKind, Read},
   path::PathBuf,
};

use futures::{stream, FutureExt, StreamExt, TryFutureExt};
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
   user, util, Env,
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
   token: &str, env: &State<Env>, client: &State<Client>,
) -> Result<Vec<DiscordGuild>, (Status, String)> {
   let guilds: Vec<DiscordGuild> = client
      .get("https://discord.com/api/v8/users/@me/guilds")
      .bearer_auth(token)
      .send()
      .then(util::deserialize)
      .await?;

   let bot_guilds: Vec<DiscordGuild> = client
      .get("https://discord.com/api/v8/users/@me/guilds")
      .bot_auth(&env.bot_token)
      .send()
      .then(util::deserialize)
      .await?;

   Ok(guilds.into_iter().filter(|guild| bot_guilds.contains(guild)).collect())
}

#[get("/guilds")]
pub async fn get_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<Client>,
) -> Result<Json<Vec<Guild>>, (Status, String)> {
   get_auth_token(env, cookies, client)
      .and_then(|token| async move { get_mutual_guilds(&token, env, client).await })
      .await
      .map(|guilds| Json(guilds.into_iter().map(|guild| guild.into()).collect()))
}

#[get("/guilds/admin")]
pub async fn get_admin_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<Client>,
) -> Result<Json<Vec<Guild>>, (Status, String)> {
   let token = get_auth_token(env, cookies, client).await?;
   let guilds = get_mutual_guilds(&token, env, client).await?;
   let user_id = user::get_current_user_id(&token, client).await?;

   // TODO: break this out into helper functions
   let admin_guilds = stream::iter(guilds)
      .filter_map(|guild| async {
         if guild.owner {
            Some(guild)
         } else {
            match client
               .get(format!(
                  "https://discord.com/api/v8/guilds/{}/members/{}",
                  guild.id, user_id,
               ))
               .bot_auth(&env.bot_token)
               .send()
               .then(util::deserialize::<DiscordGuildRoles>)
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
      .map(|guild| guild.into())
      .collect::<Vec<_>>()
      .await;

   Ok(Json(admin_guilds))
}
