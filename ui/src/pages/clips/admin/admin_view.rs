use yew::{html, Html};

impl super::Admin {
   pub(super) fn render(&self) -> Html {
      html! {
         <div>
            { "This will be the admin page" }
         </div>
      }
   }
}
