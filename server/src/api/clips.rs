use futures::TryFutureExt;
use rocket::{
   data::{ByteUnit, Data},
   http::{CookieJar, Status},
   serde::json::Json,
   State,
};
use std::{
   fs,
   path::{Component, PathBuf},
};

use super::{auth, guilds, user, ApiResponse};
use crate::{discord_client::DiscordClient, Env};
use theyrefor_models::GuildClips;

#[post("/clips/<guild_id>/<name>")]
pub async fn play_clip(
   guild_id: &str, name: &str, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<()> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| user::get_current_user_id(token, client))
      .and_then(|user_id| client.play_clip(&env.bot_uri, guild_id, user_id, name))
      .await
}

#[get("/clips/<id>")]
pub async fn get_clips(
   id: &str, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<Json<GuildClips>> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| client.get_user_guilds(token))
      .and_then(|guilds| async move {
         let guild = guilds
            .into_iter()
            .find(|guild| guild.id == id)
            .ok_or((Status::Forbidden, String::new()))?;

         let guild_dir = [&env.clip_directory, id].into_iter().collect();
         let clip_names: Vec<String> = get_clip_names(guild_dir)
            .into_iter()
            .map(|name| name.to_lowercase())
            .collect();

         let user_names = {
            let mut user_names: Vec<String> = client
               .get_guild_members(&guild.id)
               .map_ok(|members| members.into_iter().map(|member| member.user.username).collect())
               .await?;
            user_names.sort_unstable_by_key(|name| name.to_lowercase());
            user_names
         };
         let user_clip_names: Vec<String> = user_names
            .iter()
            .filter(|name| clip_names.contains(&name.to_lowercase()))
            .map(|name| name.to_owned())
            .collect();
         let user_clip_names_lower: Vec<String> = user_clip_names.iter().map(|name| name.to_lowercase()).collect();

         Ok(Json(GuildClips {
            clip_names: clip_names
               .into_iter()
               .filter(|name| !user_clip_names_lower.contains(&name.to_lowercase()))
               .collect(),
            user_clip_names,
            user_names,
            guild_name: guild.name,
         }))
      })
      .await
}

#[delete("/clips/<id>/<name>")]
pub async fn delete_clip(
   id: &str, name: &str, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<()> {
   let token = auth::get_auth_token(env, cookies, client).await?;
   let guilds = guilds::get_mutual_guilds(&token, client).await?;
   let user_id = user::get_current_user_id(&token, client).await?;

   let guild = guilds
      .into_iter()
      .find(|guild| guild.id == id)
      .ok_or((Status::Forbidden, String::new()))?;

   match guilds::take_guild_if_admin(env, client, guild, &user_id).await {
      None => Err((Status::Forbidden, String::new())),
      Some(_) => {
         let mut path: PathBuf = [&env.clip_directory, id].into_iter().collect();
         path.push(format!("{}.mp3", name.to_lowercase()));

         // Security: don't allow directory traversal attacks
         if path.components().any(|component| component == Component::ParentDir) {
            Err((Status::BadRequest, String::new()))
         } else {
            fs::remove_file(path).map_err(|_| (Status::InternalServerError, String::new()))
         }
      }
   }
}

#[put("/clips/<id>/<name>", data = "<clip>")]
pub async fn upload_clip(
   id: &str, name: &str, clip: Data<'_>, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<()> {
   let token = auth::get_auth_token(env, cookies, client).await?;
   let guilds = guilds::get_mutual_guilds(&token, client).await?;
   let user_id = user::get_current_user_id(&token, client).await?;

   let guild = guilds
      .into_iter()
      .find(|guild| guild.id == id)
      .ok_or((Status::Forbidden, String::new()))?;

   match guilds::take_guild_if_admin(env, client, guild, &user_id).await {
      None => Err((Status::Forbidden, String::new())),
      Some(_) => {
         let mut path: PathBuf = [&env.clip_directory, id].into_iter().collect();
         path.push(format!("{}.mp3", name.to_lowercase()));

         // Security: don't allow directory traversal attacks
         if path.components().any(|component| component == Component::ParentDir) {
            Err((Status::BadRequest, String::new()))
         } else {
            clip
               .open(ByteUnit::Megabyte(50))
               .into_file(path)
               .await
               .map(|_| {})
               .map_err(|_| (Status::InternalServerError, String::new()))
         }
      }
   }
}

fn get_clip_names(guild_dir: PathBuf) -> Vec<String> {
   fs::read_dir(guild_dir)
      .map(|entries| {
         let mut clips = entries
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
            .collect::<Vec<_>>();
         clips.sort_unstable_by_key(|clip| clip.to_lowercase());
         clips
      })
      .unwrap_or_else(|err| {
         error!("Could not list audio file directory: {}", err);
         Vec::new()
      })
}
