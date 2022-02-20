#[macro_use]
extern crate rocket;

mod api;
mod discord_client;
mod fairing;
mod middleware;
mod spa_server;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::ClientBuilder;
use std::env;

use discord_client::DiscordClient;
use middleware::UserAgentMiddleware;
use spa_server::SPAServer;

pub struct Env {
   base_uri: String,
   bot_uri: String,
   bot_token: String,
   client_id: String,
   client_secret: String,
   clip_directory: String,
   is_release: bool,
}

#[launch]
fn rocket() -> _ {
   let rocket = rocket::build();
   let is_release = rocket.figment().profile() == "release";
   let client = ClientBuilder::new(Client::new())
      .with(UserAgentMiddleware)
      .with(Cache(HttpCache {
         mode: CacheMode::Default,
         manager: CACacheManager::default(),
         options: None,
      }))
      .build();

   rocket
      .mount(
         "/api",
         routes![
            api::audio::get_clip,
            api::auth::authorize,
            api::auth::login,
            api::auth::logout,
            api::clips::delete_clip,
            api::clips::get_clips,
            api::clips::play_clip,
            api::guilds::get_admin_guilds,
            api::guilds::get_guilds,
            api::user::get_user
         ],
      )
      .mount("/", SPAServer::from(if is_release { "dist" } else { "../ui/dist" }))
      .attach(fairing::Cors)
      .manage(DiscordClient::new(client))
      .manage(Env {
         base_uri: env::var("BASE_URI").expect("BASE_URI must be in the environment!"),
         bot_uri: env::var("BOT_URI").expect("BOT_URI must be in the environment!"),
         bot_token: env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be in the environment!"),
         client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be in the environment!"),
         client_secret: env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be in the environment!"),
         clip_directory: env::var("CLIP_DIRECTORY").expect("CLIP_DIRECTORY must be in the environment!"),
         is_release,
      })
}
