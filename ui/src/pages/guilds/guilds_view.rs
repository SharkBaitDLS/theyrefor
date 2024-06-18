use yew::{html, Context, Html};
use yew_router::components::Link;

use crate::route::Route;

impl super::Guilds {
   pub(super) fn render(&self, _ctx: &Context<Self>) -> Html {
      match &self.guilds {
         // Loading
         None => html! {
            <div class="hero">
               <div class="lds-ellipsis container"><div></div><div></div><div></div><div></div></div>
            </div>
         },
         // Failed to load
         Some(Err(_)) => html! {
            <div class="columns is-centered mt-2 px-4">
               <article class="message is-danger">
                  <div class="message-header">
                     <p>{ "Error" }</p>
                  </div>
                  <div class="message-body">
                     { "We were unable to load your Discord servers. Please try again." }
                  </div>
               </article>
            </div>
         },
         // Success
         Some(Ok(response)) => html! {
            {
               if response.is_empty() {
                  // No servers available
                  html! {
                     <div class="columns is-centered mt-2 px-4">
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
                  // Show server icons
                  html! {
                     <div class="container">
                        <div class="columns is-centered">
                           <div class="hero">
                           {
                              if self.is_admin {
                                 html! {
                                    <div class="hero-body container">
                                       <h1 class="title is-1">{ "Servers You Manage" }</h1>
                                       <h2 class="subtitle">{ "Select one to upload or modify clips" }</h2>
                                    </div>
                                 }
                              } else {
                                 html! {
                                    <div class="hero-body container">
                                       <h1 class="title is-1">{ "Your Servers" }</h1>
                                       <h2 class="subtitle">{ "Select one to see the available clips" }</h2>
                                    </div>
                                 }
                              }
                           }
                           </div>
                        </div>
                        // TODO: fix/create different layout on mobile
                        <div class="columns is-centered">
                        {
                           for response.iter().map(|guild| {
                              let route = if self.is_admin {
                                 Route::Server { guild_id: guild.id.clone() }
                              } else {
                                 Route::Soundboard { guild_id: guild.id.clone() }
                              };
                              html! {
                                 <Link<Route> classes="py-1 px-3" to={route} >
                                 {
                                    match guild.icon.clone() {
                                       Some(icon) => html! {
                                          <p class="column has-tooltip-arrow has-tooltip-top image"
                                             data-tooltip={guild.name.clone()}>
                                             <img class="is-128x128 is-rounded" src={icon} />
                                          </p>
                                       },
                                       None => html! {
                                          <div class="column">
                                             <div class="is-flex is-align-items-center is-justify-content-center image is-128x128">
                                                <b>{ guild.name.clone() }</b>
                                             </div>
                                          </div>
                                       }
                                    }
                                 }
                                 </Link<Route>>
                              }
                           })
                        }
                        </div>
                     </div>
                  }
               }
            }
         },
      }
   }
}
