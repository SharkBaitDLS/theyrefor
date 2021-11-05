use std::{collections::BinaryHeap, fs};

use futures::{FutureExt, TryFutureExt};
use reqwest::Client;
use rocket::{
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use serde::Deserialize;
use theyrefor_models::GuildClips;

use crate::{auth::get_auth_token, util, Env};

#[derive(Deserialize)]
struct Guild {
   id: String,
   name: String,
}

#[get("/clips/<id>")]
pub async fn get_clips(
   id: String, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<Client>,
) -> Result<Json<GuildClips>, (Status, String)> {
   get_auth_token(env, cookies, client)
      .and_then(|token| {
         client
            .get("https://discord.com/api/v8/users/@me/guilds")
            .bearer_auth(token)
            .send()
            .then(util::deserialize::<Vec<Guild>>)
      })
      .await
      .and_then(|guilds| {
         let guild = guilds
            .iter()
            .find(|guild| guild.id == id)
            .ok_or((Status::Forbidden, String::new()))?;
         let guild_dir = String::from(&env.clip_directory) + "/" + &id.to_string();

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
