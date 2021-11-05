use yew::{Component, ComponentLink, Html, Properties, ShouldRender};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
   pub guild_id: String,
}

pub struct Admin;
impl Component for Admin {
   type Message = ();

   type Properties = Props;

   fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
      Self {}
   }

   fn update(&mut self, _msg: Self::Message) -> ShouldRender {
      false
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
   }

   fn view(&self) -> Html {
      self.render()
   }
}
