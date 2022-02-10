mod admin;
mod soundboard;

use crate::http_client;
use theyrefor_models::GuildClips;

pub use admin::Admin;
pub use soundboard::Soundboard;

pub enum Msg {
   Done(GuildClips),
   Play(String),
   PlaybackSuccess,
   PlaybackError,
   Fail,
}

async fn get_clips(guild_id: String) -> Msg {
   match http_client::get_with_auth(&format!("/api/clips/{}", guild_id)).await {
      Ok(Some(clips)) => Msg::Done(clips),
      _ => Msg::Fail,
   }
}
