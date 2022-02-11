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

async fn get_mutual_guilds(
   token: impl Display, env: &State<Env>, client: &State<DiscordClient>,
) -> ApiResponse<Vec<DiscordGuild>> {
   let guilds = client.get_user_guilds(token).await?;
   let bot_guilds = client.get_bot_guilds(&env.bot_token).await?;

   Ok(guilds.into_iter().filter(|guild| bot_guilds.contains(guild)).collect())
}

#[get("/guilds")]
pub async fn get_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<Json<Vec<Guild>>> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| async move { get_mutual_guilds(token, env, client).await })
      .await
      .map(|guilds| Json(guilds.into_iter().map(|guild| guild.into()).collect()))
}

#[get("/guilds/admin")]
pub async fn get_admin_guilds(
   env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<Json<Vec<Guild>>> {
   let token = auth::get_auth_token(env, cookies, client).await?;
   let guilds = get_mutual_guilds(&token, env, client).await?;
   let user_id = user::get_current_user_id(&token, client).await?;

   // TODO: break this out into helper functions
   let admin_guilds = stream::iter(guilds)
      .filter_map(|guild| async {
         if guild.owner {
            Some(guild)
         } else {
            match client.get_guild_user_roles(&env.bot_token, &guild.id, &user_id).await {
               Ok(roles) => {
                  let path: PathBuf = [&env.clip_directory, &guild.id, ".role_id"].iter().collect();

                  let mut admin_role_data = String::new();
                  if let Err(err) = File::open(&path).map(|mut file| file.read_to_string(&mut admin_role_data)) {
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
      })
      .map(|guild| guild.into())
      .collect::<Vec<_>>()
      .await;

   Ok(Json(admin_guilds))
}
