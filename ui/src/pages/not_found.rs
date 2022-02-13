use yew::{html, Component, Context, Html};

pub struct NotFound {}
impl Component for NotFound {
   type Message = ();
   type Properties = ();

   fn create(_ctx: &Context<Self>) -> Self {
      Self {}
   }

   fn view(&self, _ctx: &Context<Self>) -> Html {
      html! {
         <section class="hero is-danger is-bold is-large">
            <div class="hero-body">
               <div class="container">
                  <h1 class="title">
                     { "Page not found" }
                  </h1>
                  <h2 class="subtitle">
                     { "This page does not seem to exist" }
                  </h2>
               </div>
            </div>
         </section>
      }
   }
}
