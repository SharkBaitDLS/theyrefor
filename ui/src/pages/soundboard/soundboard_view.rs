use yew::{html, Html};

impl super::Soundboard {
   pub(super) fn render(&self) -> Html {
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
                           { "We were unable to your server clips. Please try again." }
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
               <div class="mx-4">
                  <div class="tracklist container mt-2 mb-4">
                     {
                        for response.clip_names.iter().map(|name| {
                           html! {
                              <div>
                                 <div class="box container is-flex is-align-items-center p-2">
                                    <div class="tracklist-text mr-auto">{ name }</div>
                                    <div class="ml-auto">
                                       <button class="button is-link">
                                          {"Play"}
                                       </button>
                                    </div>
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
