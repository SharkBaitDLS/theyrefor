mod auth;
mod data;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
   rocket::build().mount("/api", routes![auth::authorize, auth::logout, data::get_guilds])
}
