use web_sys::MouseEvent;
use yew::{html, Callback, Context, Html};

use crate::pages::clips;

use super::admin_component::DeleteMsg;

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
                                                <button class="ml-auto button is-small is-primary" onclick={
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
                                          html! {
                                             <td>
                                                <label class="button is-small is-fullwidth is-link is-light">
                                                   <input type="file" accept=".mp3" multiple=false hidden=true/>
                                                   <i class="fa-solid fa-plus fa-fw"></i>
                                                </label>
                                             </td>
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
                  match self.to_delete.clone() {
                     Some(Ok(name)) => html! {
                        <article class="message floating-message is-danger">
                           <div class="message-header">
                              <p>{"Delete Confirmation"}</p>
                              <button class="delete" aria-label="cancel" onclick={
                                 cancel_delete_callback(ctx)
                              }></button>
                           </div>
                           <div class="message-body is-flex is-flex-direction-column is-justify-content-center">
                              {format!("Are you sure you want to delete {}?", name)}
                              <button class="button is-danger mt-2" onclick={
                                 confirm_delete_callback(ctx)
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
                                 cancel_delete_callback(ctx)
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
            </div>
         },
      }
   }
}

fn cancel_delete_callback(ctx: &Context<super::Admin>) -> Callback<MouseEvent> {
   ctx.link().callback(|_| DeleteMsg::Success)
}

fn confirm_delete_callback(ctx: &Context<super::Admin>) -> Callback<MouseEvent> {
   ctx.link().callback(|_| DeleteMsg::Confirm)
}

fn delete_callback(ctx: &Context<super::Admin>, name: String) -> Callback<MouseEvent> {
   ctx.link().callback(move |_| DeleteMsg::Prompt(name.clone()))
}
