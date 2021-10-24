use reqwasm::Error;
use theyrefor_models::GuildClips;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

use crate::http_client;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
   pub guild_id: String,
}

async fn get_clips(guild_id: String) -> Result<Option<GuildClips>, Error> {
   http_client::get_with_auth(&format!("/api/clips/{}", guild_id)).await
}

pub enum Msg {
   Done(GuildClips),
   Fail,
}

pub struct Soundboard {
   pub(super) data: Option<Result<GuildClips, ()>>,
}
impl Component for Soundboard {
   type Message = Msg;
   type Properties = Props;

   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      link.send_future(async move {
         match get_clips(props.guild_id).await {
            Ok(Some(clips)) => Msg::Done(clips),
            _ => Msg::Fail,
         }
      });
      Self { data: None }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Done(response) => self.data = Some(Ok(response)),
         Msg::Fail => self.data = Some(Err(())),
      };
      true
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
   }

   fn view(&self) -> Html {
      self.render()
   }
}
