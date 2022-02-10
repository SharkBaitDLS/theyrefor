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
      .await
      .and_then(|guilds| {
         let guild = guilds
            .iter()
            .find(|guild| guild.id == id)
            .ok_or((Status::Forbidden, String::new()))?;
         let guild_dir = String::from(&env.clip_directory) + "/" + &id;

         let clip_names = fs::read_dir(guild_dir)
            .map(|entries| {
               entries
                  .filter_map(|maybe_entry| {
                     maybe_entry
                        .map(|entry| {
                           let path = entry.path();
                           path
                              .file_stem()
                              .filter(|_| {
                                 path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("mp3")
                              })
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
            });

         Ok(Json(GuildClips {
            clip_names: clip_names.into_sorted_vec(),
            guild_name: guild.name.clone(),
         }))
      })
}
