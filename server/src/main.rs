#[macro_use]
extern crate rocket;

use std::env;

use rocket::{
   fairing::{Fairing, Info, Kind},
   http::Header,
   Request, Response,
};

mod auth;
mod clips;
mod guilds;
mod spa_server;
mod user;
mod util;

use crate::spa_server::SPAServer;

pub struct Env {
   base_uri: String,
   bot_token: String,
   client_id: String,
   client_secret: String,
   clip_directory: String,
   is_release: bool,
}

pub struct CORS;

#[async_trait::async_trait]
impl Fairing for CORS {
   fn info(&self) -> Info {
      Info {
         name: "Add CORS headers to responses",
         kind: Kind::Response,
      }
   }

   async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
      let env = request.rocket().state::<Env>().unwrap();
      response.set_header(Header::new("Access-Control-Allow-Origin", &env.base_uri));
      response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
   }
}

#[launch]
fn rocket() -> _ {
   let rocket = rocket::build();
   let is_release = rocket.figment().profile() == "release";

   rocket
      .mount(
         "/api",
         routes![
            auth::authorize,
            auth::login,
            auth::logout,
            clips::get_clips,
            guilds::get_admin_guilds,
            guilds::get_guilds,
            user::get_user
         ],
      )
      .mount("/", SPAServer::from(if is_release { "dist" } else { "../ui/dist" }))
      .attach(CORS)
      .manage(reqwest::Client::new())
      .manage(Env {
         base_uri: env::var("BASE_URI").expect("BASE_URI must be in the environment!"),
         bot_token: env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN must be in the environment!"),
         client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be in the environment!"),
         client_secret: env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be in the environment!"),
         clip_directory: env::var("CLIP_DIRECTORY").expect("CLIP_DIRECTORY must be in the environment!"),
         is_release,
      })
}
