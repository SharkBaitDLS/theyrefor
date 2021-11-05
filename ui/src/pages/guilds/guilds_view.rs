use yew::{html, Html};

use crate::{app_route::AppRoute, AppAnchor};

impl super::Guilds {
   pub(super) fn render(&self) -> Html {
      match &self.guilds {
         // Loading
         None => html! {
            <div class="hero">
               <div class="lds-ellipsis container"><div></div><div></div><div></div><div></div></div>
            </div>
         },
         // Failed to load
         Some(Err(_)) => html! {
            <div class="tile is-ancestor columns is-centered mt-2 px-4">
               <div class="tile is-4 is-parent">
                  <div class="tile is-child">
                     <article class="message is-danger">
                        <div class="message-header">
                           <p>{ "Error" }</p>
                        </div>
                        <div class="message-body">
                           { "We were unable to load your Discord servers. Please try again." }
                        </div>
                     </article>
                  </div>
               </div>
            </div>
         },
         // Success
         Some(Ok(response)) => html! {
            <div class="tile is-ancestor is-vertical">
               <div class="tile is-child hero">
               {
                  if self.is_admin {
                     html! {
                        <div class="hero-body container pb-0">
                           <h1 class="title is-1">{ "Servers You Manage" }</h1>
                           <h2 class="subtitle">{ "Select one to upload or modify clips" }</h2>
                        </div>
                     }
                  } else {
                     html! {
                        <div class="hero-body container pb-0">
                           <h1 class="title is-1">{ "Your Servers" }</h1>
                           <h2 class="subtitle">{ "Select one to see the available clips" }</h2>
                        </div>
                     }
                  }
               }
               </div>
               {
                  if response.is_empty() {
                     html! {
                        <div class="tile is-child">
                           <article class="message is-info">
                              <div class="message-header">
                                 <p>{ "No Servers Available" }</p>
                              </div>
                              {
                                 if self.is_admin {
                                    html! {
                                       <div class="message-body">
                                          { "You do not have permissions to modify clips in any of your servers." }
                                       </div>
                                    }
                                 } else {
                                    html! {
                                       <div class="message-body">
                                          { "You do not have access to any servers with the bot enabled." }
                                       </div>
                                    }
                                 }
                              }
                           </article>
                        </div>
                     }
                  } else {
                     html! {
                        <div class="tile is-mobile is-parent container is-flex is-flex-wrap-wrap">
                        {
                           for response.iter().map(|guild| {
                              let route = if self.is_admin {
                                 AppRoute::Server(guild.id.clone())
                              } else {
                                 AppRoute::Soundboard(guild.id.clone())
                              };
                              html! {
                                 <AppAnchor classes="tile is-parent mx-3 my-3" route=route >
                                 {
                                    match guild.icon.clone() {
                                       Some(icon) => html! {
                                          <div class="tile is-child">
                                             <figure class="is-flex-mobile is-justify-content-center">
                                                <p class="image is-128x128 has-tooltip-arrow has-tooltip-top"
                                                   data-tooltip=guild.name.clone()>
                                                   <img class="is-rounded" src=icon />
                                                </p>
                                             </figure>
                                          </div>
                                       },
                                       None => html! {
                                          <div class="tile is-child is-flex is-align-items-center is-justify-content-center image is-128x128">
                                             <div><b>{ guild.name.clone() }</b></div>
                                          </div>
                                       }
                                    }
                                 }
                                 </AppAnchor>
                              }
                           })
                        }
                        </div>
                     }
                  }
               }
            </div>
         },
      }
   }
}
