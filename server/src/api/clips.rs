use futures::TryFutureExt;
use rocket::{
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use std::{collections::BinaryHeap, fs};

use super::{auth, user, ApiResponse};
use crate::{discord_client::DiscordClient, Env};
use theyrefor_models::GuildClips;

#[post("/clips/<guild_id>/<name>")]
pub async fn play_clip(
   guild_id: String, name: String, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<()> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| user::get_current_user_id(token, client))
      .and_then(|user_id| client.play_clip(&env.bot_uri, guild_id, user_id, name))
      .await
}

#[get("/clips/<id>")]
pub async fn get_clips(
   id: String, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<Json<GuildClips>> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| client.get_user_guilds(token))
      .and_then(|guilds| async move {
         let guild = guilds
            .into_iter()
            .find(|guild| guild.id == id)
            .ok_or((Status::Forbidden, String::new()))?;
         let guild_dir = String::from(&env.clip_directory) + "/" + &id;

         let guild_users: Vec<String> = client
            .get_guild_members(&env.bot_token, &guild.id)
            .map_ok(|members| {
               members
                  .into_iter()
                  .map(|member| member.user.username.to_lowercase())
                  .collect()
            })
            .await?;

         let (user_names, clip_names) = get_clip_names(guild_dir)
            .into_sorted_vec()
            .into_iter()
            .partition(|name| guild_users.contains(&name.to_lowercase()));

         Ok(Json(GuildClips {
            clip_names,
            user_names,
            guild_name: guild.name.clone(),
         }))
      })
      .await
}

fn get_clip_names(guild_dir: String) -> BinaryHeap<String> {
   fs::read_dir(guild_dir)
      .map(|entries| {
         entries
            .filter_map(|maybe_entry| {
               maybe_entry
                  .map(|entry| {
                     let path = entry.path();
                     path
                        .file_stem()
                        .filter(|_| path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("mp3"))
                        .and_then(|stem| stem.to_str())
                        .map(String::from)
                  })
                  .ok()
                  .flatten()
            })
            .collect()
      })
      .unwrap_or_else(|err| {
         error!("Could not list audio file directory: {}", err);
         BinaryHeap::new()
      })
}
