use yew_router::{components::RouterAnchor, router::Router, switch::Permissive, Switch};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
   #[to = "/clips/{guild_id}"]
   Soundboard(String),
   #[to = "/clips"]
   Guilds,
   #[to = "/page-not-found"]
   NotFound(Permissive<String>),
   #[to = "/!"]
   Home,
}

// type aliases to make life just a bit easier
pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
