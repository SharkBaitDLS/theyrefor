mod http_client;
mod pages;
mod route;

use log::debug;
use reqwasm::http::Request;
use yew::{classes, html, Component, Context, Html};
use yew_router::{components::Link, router::BrowserRouter, Switch};

use pages::{Admin, Guilds, Home, NotFound, Soundboard};
use route::Route;
use theyrefor_models::User;

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
   navbar_active: bool,
   user: Option<User>,
}
impl Component for Model {
   type Message = Msg;
   type Properties = ();

   fn create(context: &Context<Self>) -> Self {
      context.link().send_future(async { Msg::UserData(get_user().await) });
      Self {
         navbar_active: false,
         user: None,
      }
   }

   fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
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
         Msg::Logout => ctx.link().send_future(async {
            match Request::post("/api/logout").send().await {
               Ok(response) if response.status() == 200 => Msg::UserData(None),
               _ => Msg::UserData(current_user),
            }
         }),
      }
      true
   }

   fn view(&self, ctx: &Context<Self>) -> Html {
      html! {
         <BrowserRouter>
            <div class="main is-flex is-flex-direction-column">
               { self.view_nav(ctx) }

               <main>
                  <Switch<Route> render={self::switch} />
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
         </BrowserRouter>
      }
   }
}
impl Model {
   fn view_nav(&self, ctx: &Context<Self>) -> Html {
      let Self {
         navbar_active,
         ref user,
         ..
      } = *self;

      let active_class = if navbar_active { "is-active" } else { "" };

      html! {
         <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
               <Link<Route> classes="navbar-item is-size-3 my-0 py-0" to={Route::Home}>
                  { "My Man" }
               </Link<Route>>
               <a role="button" class={classes!("navbar-burger", "burger", active_class)} aria-label="menu"
                  aria-expanded={navbar_active.to_string()} onclick={ctx.link().callback(|_| Msg::ToggleNavbar)}>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
               </a>
            </div>
            <div class={classes!("navbar-menu", active_class)} onclick={ctx.link().callback(|_| Msg::DisableNavbar)}>
               {
                  if user.is_some() {
                     html! {
                        <div class="navbar-start">
                           <Link<Route> classes="navbar-item" to={Route::Clips}>
                              { "Clips" }
                           </Link<Route>>
                           <Link<Route> classes="navbar-item" to={Route::Servers}>
                              { "Manage Servers" }
                           </Link<Route>>
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
                           <div onclick={ctx.link().callback(|_| { Msg::Logout })}>
                              <Link<Route> classes="navbar-item" to={Route::Home}>
                                 { "Log Out" }
                                 <p class="has-text-grey">{ format!("(signed in as {})", &user.username) }</p>
                              </Link<Route>>
                           </div>
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
                                             <img class="is-rounded" style="max-height:100%" src={image.clone()} />
                                          </figure>
                                       }
                                    }
                                 }
                                 <div class="has-text-white ml-2">{ &user.username }</div>
                              </div>
                              <div class="navbar-dropdown">
                                 <div onclick={ctx.link().callback(|_| { Msg::Logout })}>
                                    <Link<Route> classes="navbar-item" to={Route::Home}>
                                       { "Log Out" }
                                    </Link<Route>>
                                 </div>
                              </div>
                           </div>
                        }
                     },
                     None => html! {
                        <a class="navbar-item" onclick={ctx.link().callback(|_| Msg::Login)}>{ "Log In" }</a>
                     }
                  }
               }
               </div>
            </div>
         </nav>
      }
   }
}

fn switch(route: Route) -> Html {
   match route {
      Route::Soundboard { guild_id: id } => {
         html! { <Soundboard guild_id={id} /> }
      }
      Route::Clips => {
         html! { <Guilds /> }
      }
      Route::Servers => {
         html! { <Guilds admin=true /> }
      }
      Route::Server { guild_id: id } => {
         html! { <Admin guild_id={id} /> }
      }
      Route::Home => {
         html! { <Home /> }
      }
      Route::NotFound => {
         html! { <NotFound /> }
      }
   }
}

fn main() {
   wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
   yew::Renderer::<Model>::new().render();
}
