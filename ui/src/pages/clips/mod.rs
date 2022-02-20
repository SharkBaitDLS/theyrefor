use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlMediaElement, MouseEvent};
use yew::{Callback, TargetCast};

mod admin;
mod soundboard;

use crate::http_client;
use log::error;
use theyrefor_models::GuildClips;

pub use admin::Admin;
pub use soundboard::Soundboard;

pub enum Msg {
   Done(GuildClips),
   Unauthorized,
   Fail,
}

async fn get_clips(guild_id: String) -> Msg {
   match http_client::get_with_auth(&format!("/api/clips/{}", guild_id)).await {
      Ok(Some(clips)) => Msg::Done(clips),
      Ok(None) => Msg::Unauthorized,
      _ => Msg::Fail,
   }
}

fn preview_callback() -> Callback<MouseEvent> {
   Callback::from(|event: MouseEvent| {
      if let Some(element) = event.target_dyn_into::<HtmlElement>() {
         if let Some(audio_element) = element.get_elements_by_tag_name("audio").get_with_index(0) {
            if let Ok(audio) = audio_element.dyn_into::<HtmlMediaElement>() {
               audio.set_volume(0.3);
               if audio.play().is_err() {
                  error!("Could not stream playback");
               }
            }
         }
      };
   })
}
