use yew::{html, Html};

use super::Soundboard;

impl Soundboard {
   pub(super) fn render(&self) -> Html {
      html! {
         <section class="hero is-bold is-large">
            <div class="hero-body">
               <div class="container">
                  <h1 class="title">
                     { "Hello, I will be the track list!" }
                  </h1>
                  <h2 class="subtitle">
                     { "This page does not yet exist" }
                  </h2>
               </div>
            </div>
         </section>
      }
   }
}
