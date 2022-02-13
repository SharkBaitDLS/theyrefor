use web_sys::MouseEvent;
use yew::{Callback, Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

use crate::http_client;
use theyrefor_models::GuildClips;

#[derive(Clone, Properties)]
pub struct Props {
   pub guild_id: String,
}

pub enum PlaybackMsg {
   Play(String),
   Success,
   Fail,
}

pub enum Msg {
   Data(super::Msg),
   Playback(PlaybackMsg),
}
impl From<super::Msg> for Msg {
   fn from(message: super::Msg) -> Self {
      Msg::Data(message)
   }
}
impl From<PlaybackMsg> for Msg {
   fn from(message: PlaybackMsg) -> Self {
      Msg::Playback(message)
   }
}

pub struct Soundboard {
   pub(super) data: Option<Result<GuildClips, ()>>,
   pub(super) playback_error: Option<()>,
   pub(super) link: ComponentLink<Self>,
   guild_id: String,
}
impl Component for Soundboard {
   type Message = Msg;
   type Properties = Props;

   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      link.send_future(super::get_clips(props.guild_id.clone()));
      Self {
         data: None,
         playback_error: None,
         link,
         guild_id: props.guild_id,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Self::Message::Data(data) => match data {
            super::Msg::Done(response) => self.data = Some(Ok(response)),
            super::Msg::Fail => self.data = Some(Err(())),
         },
         Self::Message::Playback(playback) => match playback {
            PlaybackMsg::Play(clip_name) => self.link.send_future(play_clip(self.guild_id.clone(), clip_name)),
            PlaybackMsg::Success => self.playback_error = None,
            PlaybackMsg::Fail => self.playback_error = Some(()),
         },
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
      self.link.callback(move |_| PlaybackMsg::Play(name.clone()))
   }
}

async fn play_clip(guild_id: String, name: String) -> PlaybackMsg {
   match http_client::post_with_auth(&format!("/api/clips/{}/{}", guild_id, name)).await {
      Ok(_) => PlaybackMsg::Success,
      Err(_) => PlaybackMsg::Fail,
   }
}
