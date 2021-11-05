use log::debug;
use reqwasm::http::Request;
use theyrefor_models::User;
use yew::{classes, html, Component, ComponentLink, Html, ShouldRender};
use yew_router::agent::RouteRequest;
use yew_router::{agent::RouteAgentDispatcher, route::Route, switch::Permissive};
use yewtil::future::LinkFuture;

mod http_client;
mod pages;
use pages::{Admin, Guilds, Home, NotFound, Soundboard};
mod app_route;
use app_route::{AppAnchor, AppRoute, AppRouter};

pub enum Msg {
   ToggleNavbar,
   DisableNavbar,
   UserData(Option<User>),
   Login,
   Logout,
}

async fn get_user() -> Option<User> {
   match Request::get("/api/user").send().await {
      Ok(response) if response.status() == 200 => response.json().await.ok(),
      _ => None,
   }
}

pub struct Model {
   link: ComponentLink<Self>,
   navbar_active: bool,
   user: Option<User>,
}
impl Component for Model {
   type Message = Msg;
   type Properties = ();

   fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
      link.send_future(async { Msg::UserData(get_user().await) });
      Self {
         link,
         navbar_active: false,
         user: None,
      }
   }

   fn update(&mut self, msg: Self::Message) -> ShouldRender {
      let current_user = self.user.clone();
      match msg {
         Msg::ToggleNavbar => self.navbar_active = !self.navbar_active,
         Msg::DisableNavbar => self.navbar_active = false,
         Msg::UserData(user) => self.user = user,
         Msg::Login => wasm_bindgen_futures::spawn_local(async {
            if let Err(err) = http_client::get_with_auth::<()>("/api/login").await {
               debug!("Failed to log in: {:?}", err);
            }
         }),
         Msg::Logout => self.link.send_future(async {
            match Request::post("/api/logout").send().await {
               Ok(response) if response.status() == 200 => Msg::UserData(None),
               _ => Msg::UserData(current_user),
            }
         }),
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
                  { " on "}
                  <a href="https://rocket.rs">{ "Rocket" }</a>
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
         ref user,
         ..
      } = *self;

      let active_class = if navbar_active { "is-active" } else { "" };

      html! {
         <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
               <AppAnchor classes="navbar-item is-size-3 my-0 py-0" route=AppRoute::Home>
                  { "My Man" }
               </AppAnchor>
               <a role="button" class=classes!("navbar-burger", "burger", active_class) aria-label="menu"
                  aria-expanded=navbar_active.to_string() onclick=link.callback(|_| Msg::ToggleNavbar)>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
               </a>
            </div>
            <div class=classes!("navbar-menu", active_class) onclick=link.callback(|_| Msg::DisableNavbar)>
               {
                  if user.is_some() {
                     html! {
                        <div class="navbar-start">
                           <AppAnchor classes="navbar-item" route=AppRoute::Clips>
                              { "Clips" }
                           </AppAnchor>
                           <AppAnchor classes="navbar-item" route=AppRoute::Servers>
                              { "Manage Servers" }
                           </AppAnchor>
                        </div>
                     }
                  } else {
                     html! {}
                  }
               }
               <div class="navbar-end">
               {
                  match user {
                     Some(user) => if navbar_active {
                        html! {
                           <a class="navbar-item" onclick=link.callback(|_| {
                              let mut router: RouteAgentDispatcher<()> = RouteAgentDispatcher::new();
                              router.send(RouteRequest::ReplaceRoute(Route::from(AppRoute::Home)));
                              Msg::Logout
                           })>{ "Log Out " }<p class="has-text-grey">{ format!("(signed in as {})", &user.username) }</p></a>
                        }
                     } else {
                        html! {
                           <div class="navbar-item has-dropdown is-hoverable">
                              <div class="navbar-item">
                                 {
                                    match &user.avatar {
                                       None => html! {},
                                       Some(image) => html! {
                                          <figure class="image is-32x32">
                                             <img class="is-rounded" style="max-height:100%" src=image.clone() />
                                          </figure>
                                       }
                                    }
                                 }
                                 <div class="has-text-white ml-2">{ &user.username }</div>
                              </div>
                              <div class="navbar-dropdown">
                                 <a class="navbar-item" onclick=link.callback(|_| {
                                    let mut router: RouteAgentDispatcher<()> = RouteAgentDispatcher::new();
                                    router.send(RouteRequest::ReplaceRoute(Route::from(AppRoute::Home)));
                                    Msg::Logout
                                 })>{ "Log Out" }</a>
                              </div>
                           </div>
                        }
                     },
                     None => html! { <a class="navbar-item" onclick=link.callback(|_| Msg::Login)>{ "Log In" }</a> }
                  }
               }
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
         AppRoute::Clips => {
            html! { <Guilds /> }
         }
         AppRoute::Servers => {
            html! { <Guilds admin=true /> }
         }
         AppRoute::Server(id) => {
            html! { <Admin guild_id=id /> }
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
