use yew::{Component, Context, Html, Properties};

use crate::http_client;
use theyrefor_models::Guild;

pub enum Msg {
   Done(Vec<Guild>),
   Unauthorized,
   Fail,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
   #[prop_or_default]
   pub admin: bool,
}

pub struct Guilds {
   pub(super) guilds: Option<Result<Vec<Guild>, ()>>,
   pub(super) is_admin: bool,
}
impl Component for Guilds {
   type Message = Msg;

   type Properties = Props;

   fn create(ctx: &Context<Self>) -> Self {
      let is_admin = ctx.props().admin;
      ctx.link().send_future(get_guilds(is_admin));
      Self { guilds: None, is_admin }
   }

   fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::Done(guilds) => self.guilds = Some(Ok(guilds)),
         Msg::Unauthorized => {}
         Msg::Fail => self.guilds = Some(Err(())),
      };
      true
   }

   fn changed(&mut self, ctx: &Context<Self>) -> bool {
      if ctx.props().admin != self.is_admin {
         self.is_admin = ctx.props().admin;
         self.guilds = None;
         ctx.link().send_future(get_guilds(self.is_admin));
      }
      true
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      self.render(ctx)
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
      Ok(None) => Msg::Unauthorized,
      Err(_) => Msg::Fail,
   }
}
