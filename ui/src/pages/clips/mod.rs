mod admin;
mod soundboard;

pub use admin::Admin;
pub use soundboard::Soundboard;
use theyrefor_models::GuildClips;

use crate::http_client;

pub enum Msg {
   Done(GuildClips),
   Fail,
}

async fn get_clips(guild_id: String) -> Msg {
   match http_client::get_with_auth(&format!("/api/clips/{}", guild_id)).await {
      Ok(Some(clips)) => Msg::Done(clips),
      _ => Msg::Fail,
   }
}
