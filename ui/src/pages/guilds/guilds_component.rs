use reqwasm::Error;
use theyrefor_models::Guild;
use yew::{Component, ComponentLink, Html, ShouldRender};
use yewtil::future::LinkFuture;

use crate::http_client;

async fn get_guilds() -> Result<Option<Vec<Guild>>, Error> {
   http_client::get("/api/guilds").await
}

pub enum Msg {
   Done(Vec<Guild>),
   Fail,
}

pub struct Guilds {
   pub(super) guilds: Option<Result<Vec<Guild>, ()>>,
}
impl Component for Guilds {
   type Message = Msg;

   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      link.send_future(async {
         match get_guilds().await {
            Ok(maybe_guilds) => match maybe_guilds {
               Some(guilds) => Msg::Done(guilds),
               None => Msg::Fail,
            },
            Err(_) => Msg::Fail,
         }
      });
      Self { guilds: None }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Done(guilds) => self.guilds = Some(Ok(guilds)),
         Msg::Fail => self.guilds = Some(Err(())),
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
