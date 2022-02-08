use theyrefor_models::GuildClips;
use web_sys::MouseEvent;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

use crate::http_client;

#[derive(Clone, Properties)]
pub struct Props {
   pub guild_id: String,
}

pub struct Soundboard {
   pub(super) data: Option<Result<GuildClips, ()>>,
   pub(super) playback_error: Option<()>,
   pub(super) link: ComponentLink<Self>,
   guild_id: String,
}
impl Component for Soundboard {
   type Message = super::Msg;
   type Properties = Props;

   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      let guild_id = props.guild_id.clone();
      link.send_future(super::get_clips(props.guild_id));
      Self {
         data: None,
         playback_error: None,
         link,
         guild_id,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Self::Message::Done(response) => self.data = Some(Ok(response)),
         Self::Message::Play(clip_name) => self.link.send_future(play_clip(self.guild_id.clone(), clip_name)),
         Self::Message::PlaybackSuccess => self.playback_error = None,
         Self::Message::PlaybackError => self.playback_error = Some(()),
         Self::Message::Fail => self.data = Some(Err(())),
      };
      true
   }

   fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if props.guild_id != self.guild_id {
         self.link.send_future(super::get_clips(props.guild_id));
         true
      } else {
         false
      }
   }

   fn view(&self) -> Html {
      self.render()
   }
}
impl Soundboard {
   pub fn playback_callback(&self, name: String) -> Callback<MouseEvent> {
      self.link.callback(move |_| super::Msg::Play(name.clone()))
   }
}

async fn play_clip(guild_id: String, name: String) -> super::Msg {
   match http_client::post_with_auth(&format!("/api/clips/{}/{}", guild_id, name)).await {
      Ok(_) => super::Msg::PlaybackSuccess,
      Err(_) => super::Msg::PlaybackError,
   }
}
