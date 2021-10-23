use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{route::Route, switch::Permissive};

mod http_client;
mod pages;
use pages::{Guilds, Home, NotFound, Soundboard};
mod app_route;
use app_route::{AppAnchor, AppRoute, AppRouter};

pub enum Msg {
   ToggleNavbar,
   DisableNavbar,
}

pub struct Model {
   link: ComponentLink<Self>,
   navbar_active: bool,
}
impl Component for Model {
   type Message = Msg;
   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      Self {
         link,
         navbar_active: false,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      match msg {
         Msg::ToggleNavbar => self.navbar_active = !self.navbar_active,
         Msg::DisableNavbar => self.navbar_active = false,
      }
      true
   }

   fn change(&mut self, _props: Self::Properties) -> ShouldRender {
      false
   }

   fn view(&self) -> Html {
      html! {
         <div class="main is-flex is-flex-direction-column">
            { self.view_nav() }

            <main>
               <AppRouter render=AppRouter::render(Self::route) redirect=AppRouter::redirect(|route: Route| {
                  AppRoute::NotFound(Permissive(Some(route.route))) }) />
            </main>
            <div class="is-flex-grow-1"/>
            <footer class="footer mt-auto">
               <div class="content has-text-centered">
                  { "Powered by " }
                  <a href="https://yew.rs">{ "Yew" }</a>
                  { " using " }
                  <a href="https://bulma.io">{ "Bulma" }</a>
               </div>
            </footer>
         </div>
      }
   }
}
impl Model {
   fn view_nav(&self) -> Html {
      let Self {
         ref link,
         navbar_active,
         ..
      } = *self;

      let active_class = if navbar_active { "is-active" } else { "" };

      html! {
         <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
               <AppAnchor classes="navbar-item is-size-3" route=AppRoute::Home>
                  { "My Man" }
               </AppAnchor>
               <a role="button" class=classes!("navbar-burger", "burger" , active_class) aria-label="menu"
                  aria-expanded=navbar_active.to_string() onclick=link.callback(|_| Msg::ToggleNavbar)>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
               </a>
            </div>
            <div class=classes!("navbar-menu", active_class) onclick=link.callback(|_| Msg::DisableNavbar)>
               <div class="navbar-start">
                  <AppAnchor classes="navbar-item" route=AppRoute::Guilds>
                     { "Clips" }
                  </AppAnchor>
               </div>
            </div>
         </nav>
      }
   }

   fn route(route: AppRoute) -> Html {
      match route {
         AppRoute::Soundboard(id) => {
            html! { <Soundboard guild_id=id /> }
         }
         AppRoute::Guilds => {
            html! { <Guilds /> }
         }
         AppRoute::Home => {
            html! { <Home /> }
         }
         AppRoute::NotFound(Permissive(route)) => {
            html! { <NotFound route=route /> }
         }
      }
   }
}

fn main() {
   wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
   yew::start_app::<Model>();
}
