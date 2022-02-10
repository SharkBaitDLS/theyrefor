mod models;
mod util;

use futures::{FutureExt, TryFutureExt};
use reqwest_middleware::ClientWithMiddleware;
use rocket::http::Status;
use serde::Serialize;
use std::fmt::Display;

use crate::api::ApiResponse;
use theyrefor_models::User;
use util::DiscordBotAuthBuilder;

pub use models::{DiscordAuthResponse, DiscordGuild, DiscordGuildRoles};

const DISCORD_API_VER: u8 = 9;

pub struct DiscordClient {
   client: ClientWithMiddleware,
}
impl DiscordClient {
   pub fn new(client: ClientWithMiddleware) -> Self {
      DiscordClient { client }
   }

   pub async fn update_token<T: Serialize>(&self, request: T) -> Result<DiscordAuthResponse, Status> {
      let body = serde_urlencoded::to_string(request).map_err(|_| {
         error!("Malformed body could not be encoded");
         Status::InternalServerError
      })?;

      self
         .client
         .post(format!("https://discord.com/api/v{}/oauth2/token", DISCORD_API_VER))
         .header(reqwest::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
         .body(body)
         .send()
         .then(util::deserialize)
         .map_err(|(status, _)| status)
         .await
   }

   pub async fn get_current_user<T: Display>(&self, token: T) -> ApiResponse<User> {
      self
         .client
         .get(format!("https://discord.com/api/v{}/users/@me", DISCORD_API_VER))
         .bearer_auth(token)
         .send()
         .then(util::deserialize)
         .await
   }

   pub async fn get_user_guilds<T: Display>(&self, token: T) -> ApiResponse<Vec<DiscordGuild>> {
      self
         .client
         .get(format!("https://discord.com/api/v{}/users/@me/guilds", DISCORD_API_VER))
         .bearer_auth(token)
         .send()
         .then(util::deserialize)
         .await
   }

   pub async fn get_guild_user_roles<T: Display>(
      &self, bot_token: T, guild_id: &str, user_id: &str,
   ) -> ApiResponse<DiscordGuildRoles> {
      self
         .client
         .get(format!(
            "https://discord.com/api/v{}/guilds/{}/members/{}",
            DISCORD_API_VER, guild_id, user_id,
         ))
         .bot_auth(bot_token)
         .send()
         .then(util::deserialize::<DiscordGuildRoles>)
         .await
   }

   pub async fn get_bot_guilds<T: Display>(&self, bot_token: T) -> ApiResponse<Vec<DiscordGuild>> {
      self
         .client
         .get(format!("https://discord.com/api/v{}/users/@me/guilds", DISCORD_API_VER))
         .bot_auth(bot_token)
         .send()
         .then(util::deserialize)
         .await
   }

   pub async fn play_clip(&self, guild_id: String, user_id: String, name: String) -> ApiResponse<()> {
      self
         .client
         .post(format!(
            "http://my-man.imgoodproductions.org:8000/play/{}/{}/{}",
            guild_id, user_id, name
         ))
         .send()
         .then(util::check_ok)
         .await
   }
}
