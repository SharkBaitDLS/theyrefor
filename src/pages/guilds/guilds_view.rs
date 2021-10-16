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
                  <div class="hero-body container pb-0">
                     <h1 class="title is-1">{ "Your Servers" }</h1>
                     <h2 class="subtitle">{ "Select one to see the available clips" }</h2>
                  </div>
               </div>

               <div class="tile is-parent container">
                  {
                     for response.iter().map(|guild| {
                        html! {
                           <AppAnchor classes="tile is-parent" route=AppRoute::Soundboard(guild.id)>
                              <div class="tile is-child card">
                                 <div class="card-content">
                                    <div class="media">
                                       <div class="media-left">
                                          <figure class="image is-128x128">
                                             <img src=guild.image_url.clone() width="128" height="128" />
                                          </figure>
                                       </div>
                                       <div class="media-content">
                                          <p class="title is-4">{ guild.name.clone() }</p>
                                       </div>
                                    </div>
                                 </div>
                              </div>
                           </AppAnchor>
                        }
                     })
                  }
               </div>
            </div>
         },
      }
   }
}
