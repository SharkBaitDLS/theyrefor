use theyrefor_models::GuildClips;
use yew::{Component, ComponentLink, Html, Properties, ShouldRender};
use yewtil::future::LinkFuture;

#[derive(Clone, Properties)]
pub struct Props {
   pub guild_id: String,
}

pub struct Soundboard {
   pub(super) data: Option<Result<GuildClips, ()>>,
   link: ComponentLink<Self>,
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
         link,
         guild_id,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Self::Message::Done(response) => self.data = Some(Ok(response)),
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
