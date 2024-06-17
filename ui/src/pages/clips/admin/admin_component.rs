use web_sys::File;
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

pub enum UploadMsg {
   File(File, String, Box<UploadMsg>),
   ClipSuccess(String),
   UserSuccess(String),
   Cancel,
   Fail(String),
}

pub enum Msg {
   Data(super::Msg),
   Delete(DeleteMsg),
   Upload(UploadMsg),
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
impl From<UploadMsg> for Msg {
   fn from(message: UploadMsg) -> Self {
      Msg::Upload(message)
   }
}

pub struct Admin {
   pub(super) data: Option<Result<GuildClips, ()>>,
   pub(super) to_delete: Option<Result<String, String>>,
   pub(super) upload: Option<Result<String, String>>,
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
         upload: None,
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
         Msg::Upload(upload) => match upload {
            UploadMsg::File(file, name, success_msg) => {
               self.upload = Some(Ok(name.clone()));
               ctx.link()
                  .send_future(upload_clip(self.guild_id.clone(), name, file, *success_msg));
            }
            UploadMsg::Cancel => {
               self.upload = None;
            }
            UploadMsg::UserSuccess(name) => {
               if let Some(Ok(data)) = &mut self.data {
                  let user_clips = &mut data.user_clip_names;
                  user_clips.push(name);
                  self.upload = None;
               }
            }
            UploadMsg::ClipSuccess(name) => {
               if let Some(Ok(data)) = &mut self.data {
                  let clip_names = &mut data.clip_names;
                  clip_names.push(name);
                  clip_names.sort_unstable_by_key(|clip| clip.to_lowercase());
                  self.upload = None;
               }
            }
            UploadMsg::Fail(name) => self.upload = Some(Err(name)),
         },
      };
      true
   }

   fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
      if ctx.props().guild_id != self.guild_id {
         self.guild_id.clone_from(&ctx.props().guild_id);
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

async fn upload_clip(guild_id: String, name: String, clip: File, success_msg: UploadMsg) -> UploadMsg {
   match http_client::put_with_auth(&format!("/api/clips/{guild_id}/{name}"), clip).await {
      Ok(_) => success_msg,
      Err(_) => UploadMsg::Fail(name),
   }
}

async fn delete_clip(guild_id: String, name: String) -> DeleteMsg {
   match http_client::delete_with_auth(&format!("/api/clips/{guild_id}/{name}")).await {
      Ok(_) => DeleteMsg::Success,
      Err(_) => DeleteMsg::Fail(name),
   }
}
