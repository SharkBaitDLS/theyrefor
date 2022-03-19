use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, MouseEvent};
use yew::{html, html::TargetCast, Callback, Context, Html};

use crate::pages::clips;

use super::admin_component::{DeleteMsg, UploadMsg};

impl super::Admin {
   pub(super) fn render(&self, ctx: &Context<Self>) -> Html {
      match &self.data {
         // Loading
         None => html! {
            <div class="hero">
               <div class="lds-ellipsis container"><div></div><div></div><div></div><div></div></div>
            </div>
         },
         // Error
         Some(Err(_)) => html! {
            <div class="tile is-ancestor columns is-centered mt-2 px-4">
               <div class="tile is-4 is-parent">
                  <div class="tile is-child">
                     <article class="message is-danger">
                        <div class="message-header">
                           <p>{ "Error" }</p>
                        </div>
                        <div class="message-body">
                           { "We were unable retrieve to your server clips. Please try again." }
                        </div>
                     </article>
                  </div>
               </div>
            </div>
         },
         // Success
         Some(Ok(response)) => html! {
            <div>
               <div class="tile is-ancestor columns is-centered mt-2 px-4">
                  <div class="tile is-4 is-parent">
                     <div class="tile is-child">
                        <div class="message is-info">
                           <div class="message-header">
                              <p>{"How it Works"}</p>
                           </div>
                           <div class="message-body">
                              {"Here, you can preview, delete, or upload clips for your server. New clips must be"}
                              {" in the MP3 format and no more than 50Mb."}
                           </div>
                        </div>
                     </div>
                  </div>
               </div>
               <div class="is-flex is-flex-direction-row is-justify-content-center is-flex-wrap-wrap">
                  <div class="m-3 is-flex is-flex-direction-column is-justify-content-flex-start">
                     <div class="mb-1 is-size-5 has-text-centered">
                        { format!("User Entrance Clips for {}", response.guild_name) }
                     </div>
                     <table class="table is-bordered is-striped">
                        <thead>
                           <tr>
                              <th>{ "User Name" }</th>
                              <th>{ "Clip" }</th>
                           </tr>
                        </thead>
                        <tbody>
                           {
                              for response.user_names.iter().map(|name| html! {
                                 <tr>
                                    <td class="tracklist-text">{name}</td>
                                    {
                                       if response.user_clip_names.contains(name) {
                                          html! {
                                             <td>
                                                <button class="ml-auto button is-small is-primary"
                                                   onclick={
                                                      clips::preview_callback()
                                                   }>
                                                   <audio controls=false preload="none">
                                                      <source type="audio/mpeg" src={
                                                         format!("/api/audio/{}/{}", ctx.props().guild_id, name)
                                                      }/>
                                                   </audio>
                                                   <i class="fa-solid fa-headphones fa-fw"/>
                                                </button>
                                                <button class="ml-1 button is-small is-danger" onclick={
                                                   delete_callback(ctx, name.to_owned())
                                                }>
                                                   <i class="fa-solid fa-trash-can fa-fw"/>
                                                </button>
                                             </td>
                                          }
                                       } else {
                                          match &self.upload {
                                             Some(Ok(upload_name)) if name == upload_name => {
                                                html! {
                                                   <td>
                                                      <button class="button is-small is-fullwidth is-link is-loading"/>
                                                   </td>
                                                }
                                             },
                                             _ => {
                                                html! {
                                                   <td>
                                                      <label class="button is-small is-fullwidth is-link is-light">
                                                         <input type="file" accept=".mp3" multiple=false hidden=true
                                                            onchange={
                                                               upload_user_clip_callback(ctx, name.to_owned())
                                                            }/>
                                                         <i class="fa-solid fa-plus fa-fw"></i>
                                                      </label>
                                                   </td>
                                                }
                                             }
                                          }
                                       }
                                    }
                                 </tr>
                              })
                           }
                        </tbody>
                     </table>
                  </div>
                  <div class="m-3 is-flex is-flex-direction-column is-justify-content-flex-start">
                     <div class="mb-1 is-size-5 has-text-centered">
                        { format!("Soundboard Clips for {}", response.guild_name) }
                     </div>
                     <table class="table is-bordered is-striped">
                        <thead>
                           <tr>
                              <th>{ "Name" }</th>
                              <th>{ "Clip" }</th>
                           </tr>
                        </thead>
                        <tbody>
                           <tr>
                              <td>
                                 <input id="clip-input" class="input is-small" type="text" spellcheck="false"
                                    placeholder="Clip name"/>
                              </td>
                              <td>
                                 <label class="button is-small is-link is-light is-fullwidth">
                                    <input type="file" accept=".mp3" multiple=false hidden=true
                                       onchange={upload_clip_form_callback(ctx)}/>
                                    <i class="fa-solid fa-plus fa-fw"></i>
                                 </label>
                              </td>
                           </tr>
                           {
                              match &self.upload {
                                 Some(Ok(upload_name)) if !response.user_clip_names.contains(&upload_name.to_lowercase()) => html! {
                                    <tr>
                                       <td class="tracklist-text">{upload_name}</td>
                                       <td>
                                          <button class="button is-small is-fullwidth is-link is-loading"/>
                                       </td>
                                    </tr>
                                 },
                                 _ => html! {}
                              }
                           }
                           {
                              for response.clip_names.iter().map(|name| html! {
                                 <tr>
                                    <td class="tracklist-text">{name}</td>
                                    <td>
                                       <button class="ml-auto button is-small is-primary" onclick={
                                          clips::preview_callback()
                                       }>
                                          <audio controls=false preload="none">
                                             <source src={format!("/api/audio/{}/{}", ctx.props().guild_id, name)}
                                                   type="audio/mpeg"/>
                                          </audio>
                                          <i class="fa-solid fa-headphones fa-fw"/>
                                       </button>
                                       <button class="ml-1 button is-small is-danger" onclick={
                                          delete_callback(ctx, name.to_owned())
                                       }>
                                          <i class="fa-solid fa-trash-can fa-fw"/>
                                       </button>
                                    </td>
                                 </tr>
                              })
                           }
                        </tbody>
                     </table>
                  </div>
               </div>
               {
                  match &self.to_delete {
                     Some(Ok(name)) => html! {
                        <article class="message floating-message is-danger">
                           <div class="message-header">
                              <p>{"Delete Confirmation"}</p>
                              <button class="delete" aria-label="cancel" onclick={
                                 ctx.link().callback(|_| DeleteMsg::Success)
                              }></button>
                           </div>
                           <div class="message-body is-flex is-flex-direction-column is-justify-content-center">
                              {format!("Are you sure you want to delete {}?", name)}
                              <button class="button is-danger mt-2" onclick={
                                 ctx.link().callback(|_| DeleteMsg::Confirm)
                              }>
                                 { "Yes, really delete" }
                              </button>
                           </div>
                        </article>
                     },
                     Some(Err(name)) => html! {
                        <article class="message floating-message is-danger">
                           <div class="message-header">
                              <p>{"Delete Failed"}</p>
                              <button class="delete" aria-label="cancel" onclick={
                                 ctx.link().callback(|_| DeleteMsg::Success)
                              }></button>
                           </div>
                           <div class="message-body is-flex is-flex-direction-column is-justify-content-center">
                              {format!("Failed to delete {}.", name)}
                           </div>
                        </article>
                     },
                     None =>  html! {}
                  }
               }
               {
                  match &self.upload {
                     Some(Err(name)) => html! {
                        <article class="message floating-message is-danger">
                           <div class="message-header">
                              <p>{"Upload Failed"}</p>
                              <button class="delete" aria-label="cancel" onclick={
                                 ctx.link().callback(|_| UploadMsg::Cancel)
                              }></button>
                           </div>
                           <div class="message-body is-flex is-flex-direction-column is-justify-content-center">
                              {format!("Failed to upload {}.", name)}
                           </div>
                        </article>
                     },
                     _ => html!{},
                  }
               }
            </div>
         },
      }
   }
}

fn upload_user_clip_callback(ctx: &Context<super::Admin>, name: String) -> Callback<Event> {
   ctx.link().callback(move |event: Event| {
      let input: HtmlInputElement = event.target_unchecked_into();
      input
         .files()
         .and_then(|files| files.item(0))
         .map(|file| UploadMsg::File(file, name.to_owned(), Box::new(UploadMsg::UserSuccess(name.to_owned()))))
         .unwrap_or(UploadMsg::Cancel)
   })
}

fn upload_clip_form_callback(ctx: &Context<super::Admin>) -> Callback<Event> {
   ctx.link().callback(move |event: Event| {
      let input: HtmlInputElement = event.target_unchecked_into();

      let result = input
         .owner_document()
         .and_then(|document| document.get_element_by_id("clip-input"))
         .and_then(|element| {
            element
               .dyn_into::<HtmlInputElement>()
               .map(|text_input| {
                  let name = text_input.value();
                  text_input.set_value("");
                  name
               })
               .ok()
               .filter(|name| !name.trim().is_empty())
               .and_then(|name| {
                  input
                     .files()
                     .and_then(|files| files.item(0))
                     .filter(|_| !name.trim().is_empty())
                     .map(|file| UploadMsg::File(file, name.to_owned(), Box::new(UploadMsg::ClipSuccess(name))))
               })
         })
         .unwrap_or(UploadMsg::Cancel);

      input.set_value("");
      result
   })
}

fn delete_callback(ctx: &Context<super::Admin>, name: String) -> Callback<MouseEvent> {
   ctx.link().callback(move |_| DeleteMsg::Prompt(name.to_owned()))
}
