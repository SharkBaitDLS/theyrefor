use futures::{stream, StreamExt, TryFutureExt};
use log::error;
use rocket::{http::CookieJar, serde::json::Json, State};
use std::{
   fmt::Display,
   fs::File,
   io::{ErrorKind, Read},
   path::PathBuf,
};

use super::{auth, user, ApiResponse};
use crate::{
   discord_client::{DiscordClient, DiscordGuild},
   Env,
};
use theyrefor_models::Guild;

#[get("/guilds")]
pub async fn get_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<Json<Vec<Guild>>> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| async move { get_mutual_guilds(token, client).await })
      .await
      .map(|guilds| Json(guilds.into_iter().map(|guild| guild.into()).collect()))
}

#[get("/guilds/admin")]
pub async fn get_admin_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<Json<Vec<Guild>>> {
   let token = auth::get_auth_token(env, cookies, client).await?;
   let guilds = get_mutual_guilds(&token, client).await?;
   let user_id = user::get_current_user_id(&token, client).await?;

   let admin_guilds = stream::iter(guilds)
      .filter_map(|guild| take_guild_if_admin(env, client, guild, &user_id))
      .map(|guild| guild.into())
      .collect::<Vec<_>>()
      .await;

   Ok(Json(admin_guilds))
}

pub async fn get_mutual_guilds(token: impl Display, client: &State<DiscordClient>) -> ApiResponse<Vec<DiscordGuild>> {
   let guilds = client.get_user_guilds(token).await?;
   let bot_guilds = client.get_bot_guilds().await?;

   Ok(guilds
      .into_iter()
      .filter(|guild| bot_guilds.iter().any(|bot_guild| bot_guild.id == guild.id))
      .collect())
}

pub async fn take_guild_if_admin(
   env: &State<Env>, client: &State<DiscordClient>, guild: DiscordGuild, user_id: &str,
) -> Option<DiscordGuild> {
   if guild.owner {
      Some(guild)
   } else {
      match client.get_guild_user_roles(&guild.id, user_id).await {
         Ok(roles) => {
            let path: PathBuf = [&env.clip_directory, &guild.id, ".role_id"].iter().collect();

            let mut admin_role_data = String::new();
            if let Err(err) = File::open(path).map(|mut file| file.read_to_string(&mut admin_role_data)) {
               if err.kind() != ErrorKind::NotFound {
                  error!("Could not retrieve role ID for guild {:?}: {:?}", guild.id, err);
               }
            }

            if roles.contains(&admin_role_data) {
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
}
