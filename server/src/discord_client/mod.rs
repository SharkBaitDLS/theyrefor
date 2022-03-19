mod http_util;
mod models;
mod twilight_util;

use futures::{FutureExt, TryFutureExt};
use http::{header, HeaderMap, HeaderValue};
use moka::future::Cache;
use rocket::http::Status;
use serde::Serialize;
use std::{fmt::Display, str::FromStr, sync::Arc, time::Duration};
use twilight_http::{client::ClientBuilder, Client};
use twilight_model::id::Id;

use crate::api::ApiResponse;
use theyrefor_models::User;

pub use models::{DiscordAuthResponse, DiscordGuild, DiscordGuildMember};

const DISCORD_API_VER: u8 = 9;
const USER_AGENT: &str = "theyrefor-server/0.1";

pub struct DiscordClient {
   bot_token: String,
   http: reqwest::Client,
   user_clients: Cache<String, Arc<Client>>,
}
impl DiscordClient {
   pub fn new(bot_token: String) -> Self {
      DiscordClient {
         bot_token,
         http: reqwest::Client::new(),
         user_clients: Cache::builder()
            .max_capacity(100)
            .time_to_idle(Duration::from_secs(600))
            .build(),
      }
   }

   pub async fn update_token<T: Serialize>(&self, request: T) -> Result<DiscordAuthResponse, Status> {
      let body = serde_urlencoded::to_string(request).map_err(|_| {
         error!("Malformed body could not be encoded");
         Status::InternalServerError
      })?;

      self
         .http
         .post(format!("https://discord.com/api/v{}/oauth2/token", DISCORD_API_VER))
         .header(
            header::CONTENT_TYPE,
            mime::APPLICATION_WWW_FORM_URLENCODED.essence_str(),
         )
         .header(header::USER_AGENT, USER_AGENT)
         .body(body)
         .send()
         .then(http_util::deserialize)
         .map_err(|(status, _)| status)
         .await
   }

   pub async fn get_current_user<T: Display>(&self, token: T) -> ApiResponse<User> {
      self
         .get_user_client(token)
         .await
         .current_user()
         .exec()
         .then(twilight_util::marshal)
         .await
   }

   pub async fn get_user_guilds<T: Display>(&self, token: T) -> ApiResponse<Vec<DiscordGuild>> {
      self
         .get_user_client(token)
         .await
         .current_user_guilds()
         .exec()
         .then(twilight_util::marshal_list)
         .await
   }

   pub async fn get_guild_members(&self, guild_id: &str) -> ApiResponse<Vec<DiscordGuildMember>> {
      self
         .get_bot_client()
         .await
         .guild_members(Id::from_str(guild_id).unwrap())
         .limit(1_000)
         .unwrap()
         .exec()
         .then(twilight_util::marshal_members)
         .await
   }

   pub async fn get_guild_user_roles(&self, guild_id: &str, user_id: &str) -> ApiResponse<Vec<String>> {
      self
         .get_bot_client()
         .await
         .guild_member(Id::from_str(guild_id).unwrap(), Id::from_str(user_id).unwrap())
         .exec()
         .then(twilight_util::marshal_member::<DiscordGuildMember>)
         .map_ok(|member| member.roles)
         .await
   }

   pub async fn get_bot_guilds(&self) -> ApiResponse<Vec<DiscordGuild>> {
      self
         .get_bot_client()
         .await
         .current_user_guilds()
         .exec()
         .then(twilight_util::marshal_list)
         .await
   }

   pub async fn play_clip(&self, bot_uri: &str, guild_id: String, user_id: String, name: String) -> ApiResponse<()> {
      self
         .http
         .post(format!("{}/play/{}/{}/{}", bot_uri, guild_id, user_id, name))
         .send()
         .then(http_util::check_ok)
         .await
   }

   async fn get_bot_client(&self) -> Arc<Client> {
      self.get_or_insert_client(&self.bot_token, true).await
   }

   async fn get_user_client<T: Display>(&self, token: T) -> Arc<Client> {
      self.get_or_insert_client(token, false).await
   }

   async fn get_or_insert_client<T: Display>(&self, token: T, is_bot: bool) -> Arc<Client> {
      self
         .user_clients
         .get_or_insert_with(token.to_string(), async {
            let mut headers = HeaderMap::new();
            headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

            Arc::new(
               ClientBuilder::new()
                  .default_headers(headers)
                  .remember_invalid_token(false)
                  .token(if is_bot {
                     format!("Bot {}", token)
                  } else {
                     format!("Bearer {}", token)
                  })
                  .build(),
            )
         })
         .await
   }
}
