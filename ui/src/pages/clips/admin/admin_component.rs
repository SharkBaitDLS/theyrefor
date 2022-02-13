use yew::{Component, Context, Html, Properties};

use theyrefor_models::GuildClips;
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
   pub guild_id: String,
}

pub struct Admin {
   pub(super) data: Option<Result<GuildClips, ()>>,
   guild_id: String,
}

impl Component for Admin {
   type Message = super::Msg;

   type Properties = Props;

   fn create(ctx: &Context<Self>) -> Self {
      ctx.link().send_future(super::get_clips(ctx.props().guild_id.clone()));
      Self {
         data: None,
         guild_id: ctx.props().guild_id.clone(),
      }
   }

   fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Self::Message::Done(response) => self.data = Some(Ok(response)),
         Self::Message::Fail => self.data = Some(Err(())),
      };
      true
   }

   fn changed(&mut self, ctx: &Context<Self>) -> bool {
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
