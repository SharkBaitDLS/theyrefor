use std::{collections::BinaryHeap, fs};

use futures::TryFutureExt;
use reqwest::Client;
use rocket::{
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use serde::Deserialize;
use theyrefor_models::GuildClips;

use crate::{
   auth::{get_auth_token, DiscordBotAuthBuilder},
   Env,
};

#[derive(Deserialize)]
struct Guild {
   name: String,
}

#[get("/clips/<id>")]
pub async fn get_clips(
   id: u64, cookies: &CookieJar<'_>, client: &State<Client>, env: &State<Env>,
) -> Result<Json<GuildClips>, (Status, String)> {
   match get_auth_token(cookies, client, env).await {
      Err(redirect) => Err(redirect),
      Ok(_) => {
         let guild: Guild = match client
            .get(format!("https://discord.com/api/v8/guilds/{}", id))
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

         let guild_dir = String::from(&env.clip_directory) + "/" + &id.to_string();

         // TODO: shared library with the bot?
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
            guild_name: guild.name,
         }))
      }
   }
}
