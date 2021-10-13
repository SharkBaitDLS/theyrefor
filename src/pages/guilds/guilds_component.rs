use gloo_timers::future::TimeoutFuture;
use reqwest::Error;
use yew::{Component, ComponentLink, Html, ShouldRender};
use yewtil::future::LinkFuture;

// TODO: move to shared server models
pub struct Guild {
   pub(super) name: String,
   pub(super) id: u64,
   pub(super) image_url: String,
}

async fn get_guilds() -> Result<Vec<Guild>, Error> {
   // TODO: actually implement with HTTP
   TimeoutFuture::new(2_000).await;
   Ok::<Vec<Guild>, Error>(vec![
      Guild {
         name: "Lamer Gamers".to_string(),
         id: 1,
         image_url: "https://cdn.discordapp.com/icons/82228700078153728/de0d9079caf52e6d24f98b78c8f21871.png"
            .to_string(),
      },
      Guild {
         name: "Lamer Gamers 2".to_string(),
         id: 2,
         image_url: "https://cdn.discordapp.com/icons/82228700078153728/de0d9079caf52e6d24f98b78c8f21871.png"
            .to_string(),
      },
   ])
}

pub enum Msg {
   Done(Vec<Guild>),
   Fail,
}

pub struct Guilds {
   pub(super) loading: bool,
   pub(super) error: bool,
   pub(super) guilds: Vec<Guild>,
}
impl Component for Guilds {
   type Message = Msg;

   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      link.send_future(async {
         match get_guilds().await {
            Ok(guilds) => Msg::Done(guilds),
            Err(_) => Msg::Fail,
         }
      });
      Self {
         loading: true,
         error: false,
         guilds: Vec::new(),
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Done(guilds) => {
            self.guilds = guilds;
            self.loading = false;
         }
         Msg::Fail => {
            self.loading = false;
            self.error = true;
         }
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
