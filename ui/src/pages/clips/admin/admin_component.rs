use yew::{Component, Context, Html, Properties};

use theyrefor_models::GuildClips;

use crate::http_client;
#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
   pub guild_id: String,
}

pub enum DeleteMsg {
   Prompt(String),
   Confirm,
   Success,
   Fail(String),
}
pub enum Msg {
   Data(super::Msg),
   Delete(DeleteMsg),
}
impl From<super::Msg> for Msg {
   fn from(message: super::Msg) -> Self {
      Msg::Data(message)
   }
}
impl From<DeleteMsg> for Msg {
   fn from(message: DeleteMsg) -> Self {
      Msg::Delete(message)
   }
}

pub struct Admin {
   pub(super) data: Option<Result<GuildClips, ()>>,
   pub(super) to_delete: Option<Result<String, String>>,
   guild_id: String,
}

impl Component for Admin {
   type Message = Msg;

   type Properties = Props;

   fn create(ctx: &Context<Self>) -> Self {
      ctx.link().send_future(super::get_clips(ctx.props().guild_id.clone()));
      Self {
         data: None,
         to_delete: None,
         guild_id: ctx.props().guild_id.clone(),
      }
   }

   fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
      match msg {
         Msg::Data(data) => match data {
            super::Msg::Done(response) => self.data = Some(Ok(response)),
            super::Msg::Unauthorized => {}
            super::Msg::Fail => self.data = Some(Err(())),
         },
         Msg::Delete(delete) => match delete {
            DeleteMsg::Prompt(name) => self.to_delete = Some(Ok(name)),
            DeleteMsg::Fail(name) => self.to_delete = Some(Err(name)),
            DeleteMsg::Success => self.to_delete = None,
            DeleteMsg::Confirm => {
               if let Some(Ok(name)) = self.to_delete.clone() {
                  if let Some(Ok(data)) = &mut self.data {
                     let clips = &mut data.clip_names;
                     clips.retain(|clip| clip != &name);

                     let user_clips = &mut data.user_clip_names;
                     user_clips.retain(|clip| clip != &name);

                     ctx.link().send_future(delete_clip(self.guild_id.clone(), name));
                  }
               }
            }
         },
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

async fn delete_clip(guild_id: String, name: String) -> DeleteMsg {
   match http_client::delete_with_auth(&format!("/api/clips/{}/{}", guild_id, name)).await {
      Ok(_) => DeleteMsg::Success,
      Err(_) => DeleteMsg::Fail(name),
   }
}
