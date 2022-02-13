use yew::{html, Context, Html};

impl super::Admin {
   pub(super) fn render(&self, _ctx: &Context<Self>) -> Html {
      html! {
         <div>
            { "This will be the admin page" }
         </div>
      }
   }
}
