use std::env;

mod auth;
mod data;

#[macro_use]
extern crate rocket;

pub struct Env {
   base_uri: String,
   client_id: String,
   client_secret: String,
}

#[launch]
fn rocket() -> _ {
   rocket::build()
      .mount("/api", routes![auth::authorize, auth::logout, data::get_guilds])
      .manage(reqwest::Client::new())
      .manage(Env {
         base_uri: env::var("BASE_URI").expect("BASE_URI must be in the environment"),
         client_id: env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID must be in the environment"),
         client_secret: env::var("DISCORD_CLIENT_SECRET").expect("DISCORD_CLIENT_SECRET must be in the environment"),
      })
}
