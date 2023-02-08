use yew::{Component, Context, Html, Properties};

use crate::http_client;
use theyrefor_models::GuildClips;

#[derive(Clone, PartialEq, Eq, Properties)]
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
   guild_id: String,
}
impl Component for Soundboard {
   type Message = Msg;
   type Properties = Props;

   fn create(ctx: &Context<Self>) -> Self {
      ctx.link().send_future(super::get_clips(ctx.props().guild_id.clone()));
      Self {
         data: None,
         playback_error: None,
         guild_id: ctx.props().guild_id.clone(),
      }
   }

   fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Self::Message::Data(data) => match data {
            super::Msg::Done(response) => self.data = Some(Ok(response)),
            super::Msg::Unauthorized => {}
            super::Msg::Fail => self.data = Some(Err(())),
         },
         Self::Message::Playback(playback) => match playback {
            PlaybackMsg::Play(clip_name) => ctx.link().send_future(play_clip(self.guild_id.clone(), clip_name)),
            PlaybackMsg::Success => self.playback_error = None,
            PlaybackMsg::Fail => self.playback_error = Some(()),
         },
      };
      true
   }

   fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
      if ctx.props().guild_id != self.guild_id {
         self.guild_id = ctx.props().guild_id.clone();
         ctx.link().send_future(super::get_clips(ctx.props().guild_id.clone()));
         true
      } else {
         false
      }
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      self.render(ctx)
   }
}

async fn play_clip(guild_id: String, name: String) -> PlaybackMsg {
   match http_client::post_with_auth(&format!("/api/clips/{guild_id}/{name}")).await {
      Ok(Some(())) => PlaybackMsg::Success,
      _ => PlaybackMsg::Fail,
   }
}
