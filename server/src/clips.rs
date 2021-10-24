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
         Ok(Json(GuildClips {
            // TODO: read off of file system
            clip_names: Vec::new(),
            guild_name: guild.name,
         }))
      }
   }
}
