use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
   pub guild_id: u64,
}

pub struct Soundboard;
impl Component for Soundboard {
   type Message = ();
   type Properties = Props;

   fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
      Self
   }

   fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      unimplemented!()
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      unimplemented!()
   }

   fn view(&self) -> Html {
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
