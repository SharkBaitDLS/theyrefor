use gloo_timers::future::TimeoutFuture;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Error;
use theyrefor_models::GuildClips;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
   pub guild_id: u64,
}

// TODO: actually implement with HTTP
async fn get_clips(_guild_id: u64) -> Result<GuildClips, Error> {
   let mut result = Vec::new();

   for _ in 1..1000 {
      let rand_string: String = rand::thread_rng()
         .sample_iter(&Alphanumeric)
         .take(30)
         .map(char::from)
         .collect();
      result.push(rand_string);
   }

   TimeoutFuture::new(2_000).await;

   Ok(GuildClips {
      clip_names: result,
      guild_name: "Lamer Gamers".to_string(),
   })
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
            Ok(clips) => Msg::Done(clips),
            Err(_) => Msg::Fail,
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
