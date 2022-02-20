use web_sys::MouseEvent;
use yew::{html, Callback, Context, Html};

use crate::pages::clips;

use super::soundboard_component::PlaybackMsg;

impl super::Soundboard {
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
               <section class="section is-size-3 has-text-centered">
                  { format!("Clips for {}", response.guild_name) }
               </section>
               {
                  if self.playback_error.is_some() {
                     html! {
                        <div class="tile is-ancestor columns is-centered mt-2 px-4">
                           <div class="tile is-4 is-parent">
                              <div class="tile is-child">
                                 <article class="message is-danger">
                                    <div class="message-header">
                                       <p>{ "Error" }</p>
                                    </div>
                                    <div class="message-body">
                                       { "Could not play clip. Make sure you are in a voice channel in this server." }
                                    </div>
                                 </article>
                              </div>
                           </div>
                        </div>
                     }
                  } else {
                     html! {}
                  }
               }
               <div class="mx-4">
                  <div class="tracklist container mt-2 mb-4">
                     {
                        for response.clip_names.iter().map(|name| {
                           html! {
                              <div>
                                 <div class="box container is-flex is-align-items-center p-2">
                                    <div class="tracklist-text mr-auto">{ name }</div>
                                    <button class="ml-auto button is-small is-primary" onclick={
                                       clips::preview_callback()
                                    }>
                                       <audio controls=false preload="none">
                                          <source src={format!("/api/audio/{}/{}", ctx.props().guild_id, name)}
                                                  type="audio/mpeg"/>
                                       </audio>
                                       <i class="fa-solid fa-headphones fa-fw"/>
                                    </button>
                                    <button class="button is-small ml-1 is-link" onclick={
                                       playback_callback(ctx, name.to_string())
                                    }>
                                       <i class="fa-solid fa-play fa-fw"/>
                                    </button>
                                 </div>
                              </div>
                           }
                        })
                     }
                  </div>
               </div>
               <section class="section is-size-3 has-text-centered">
                  { format!("User entrances for {}", response.guild_name) }
               </section>
               <div class="mx-4">
                  <div class="tracklist container mt-2 mb-4">
                     {
                        for response.user_clip_names.iter().map(|name| {
                           html! {
                              <div>
                                 <div class="box container is-flex is-align-items-center p-2">
                                    <div class="tracklist-text mr-auto">{ name }</div>
                                    <button class="ml-auto button is-small is-primary" onclick={
                                       clips::preview_callback()
                                    }>
                                       <audio controls=false preload="none">
                                          <source src={format!("/api/audio/{}/{}", ctx.props().guild_id, name)}
                                                  type="audio/mpeg"/>
                                       </audio>
                                       <i class="fa-solid fa-headphones fa-fw"/>
                                    </button>
                                    <button class="button is-small ml-1 is-link" onclick={
                                       playback_callback(ctx, name.to_string())
                                    }>
                                       <i class="fa-solid fa-play fa-fw"/>
                                    </button>
                                 </div>
                              </div>
                           }
                        })
                     }
                  </div>
               </div>
            </div>
         },
      }
   }
}

fn playback_callback(ctx: &Context<super::Soundboard>, name: String) -> Callback<MouseEvent> {
   ctx.link().callback(move |_| PlaybackMsg::Play(name.clone()))
}
