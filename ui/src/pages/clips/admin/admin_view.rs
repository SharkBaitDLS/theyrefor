use yew::{html, Context, Html};

impl super::Admin {
   pub(super) fn render(&self, _ctx: &Context<Self>) -> Html {
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
                                          <td>{ "Clip stuff here" }</td>
                                       }
                                    } else {
                                       html! {
                                          <td>{ "Upload stuff here" }</td>
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
                                 <td>{"Clip stuff here"}</td>
                              </tr>
                           })
                        }
                     </tbody>
                  </table>
               </div>
            </div>
         },
      }
   }
}
