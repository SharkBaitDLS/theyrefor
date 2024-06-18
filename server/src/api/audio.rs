use std::{
   io::ErrorKind,
   path::{Component, PathBuf},
};

use futures::TryFutureExt;
use rocket::{
   fs::NamedFile,
   http::{CookieJar, Status},
   State,
};

use crate::{api::ApiResponse, discord_client::DiscordClient, Env};

use super::auth;

#[get("/audio/<guild_id>/<name>")]
pub async fn get_clip(
   guild_id: &str, name: &str, env: &State<Env>, cookies: &CookieJar<'_>, client: &State<DiscordClient>,
) -> ApiResponse<NamedFile> {
   auth::get_auth_token(env, cookies, client)
      .and_then(|token| client.get_user_guilds(token))
      .and_then(|guilds| async move {
         let guild = guilds
            .into_iter()
            .find(|guild| guild.id == guild_id)
            .ok_or((Status::Forbidden, String::new()))?;

         let guild_dir = String::from(&env.clip_directory) + "/" + &guild.id;
         let mut path = PathBuf::from(guild_dir);
         path.push(&name.to_lowercase());

         // Security: don't allow directory traversal attacks
         if path.components().any(|component| component == Component::ParentDir) {
            Err((Status::BadRequest, String::new()))
         } else {
            NamedFile::open(path).await.map_err(|err| match err.kind() {
               ErrorKind::NotFound => (Status::NotFound, String::new()),
               _ => (Status::InternalServerError, String::new()),
            })
         }
      })
      .await
}
