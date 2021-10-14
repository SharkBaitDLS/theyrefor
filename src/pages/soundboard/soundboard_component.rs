use rand::{distributions::Alphanumeric, Rng};
use reqwest::Error;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
   pub guild_id: u64,
}

async fn get_clips(guild_id: u64) -> Result<Vec<String>, Error> {
   let mut result = Vec::new();

   for _ in 1..1000 {
      let rand_string: String = rand::thread_rng()
         .sample_iter(&Alphanumeric)
         .take(10)
         .map(char::from)
         .collect();
      result.push(rand_string);
   }
   Ok(result)
}

pub enum Msg {
   Done(Vec<String>),
   Fail,
}

pub struct Soundboard {
   pub clip_names: Vec<String>,
   pub loading: bool,
   pub error: bool,
}
impl Component for Soundboard {
   type Message = Msg;
   type Properties = Props;

   fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
      link.send_future(async move {
         match get_clips(props.guild_id).await {
            Ok(clips) => Msg::Done(clips),
            Err(_) => Msg::Fail,
         }
      });
      Self {
         clip_names: Vec::new(),
         loading: true,
         error: false,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::Done(clips) => {
            self.clip_names = clips;
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
