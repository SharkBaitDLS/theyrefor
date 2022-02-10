use yew::{Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

use crate::http_client;
use theyrefor_models::Guild;

pub enum Msg {
   Done(Vec<Guild>),
   Fail,
}

#[derive(Clone, Properties)]
pub struct Props {
   #[prop_or_default]
   pub admin: bool,
}

pub struct Guilds {
   pub(super) guilds: Option<Result<Vec<Guild>, ()>>,
   pub(super) is_admin: bool,
   link: ComponentLink<Self>,
}
impl Component for Guilds {
   type Message = Msg;

   type Properties = Props;

   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      let is_admin = props.admin;
      link.send_future(get_guilds(is_admin));
      Self {
         guilds: None,
         is_admin,
         link,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Done(guilds) => self.guilds = Some(Ok(guilds)),
         Msg::Fail => self.guilds = Some(Err(())),
      };
      true
   }

   fn change(&mut self, props: Self::Properties) -> ShouldRender {
      if props.admin != self.is_admin {
         self.is_admin = props.admin;
         self.guilds = None;
         self.link.send_future(get_guilds(self.is_admin));
      }
      true
   }

   fn view(&self) -> Html {
      self.render()
   }
}

async fn get_guilds(is_admin: bool) -> Msg {
   let data = if is_admin {
      http_client::get_with_auth("/api/guilds/admin").await
   } else {
      http_client::get_with_auth("/api/guilds").await
   };
   match data {
      Ok(Some(guilds)) => Msg::Done(guilds),
      _ => Msg::Fail,
   }
}
