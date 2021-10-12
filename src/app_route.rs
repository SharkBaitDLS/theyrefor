use yew_router::{components::RouterAnchor, prelude::*, switch::Permissive};

#[derive(Clone, Debug, Switch)]
pub enum AppRoute {
    #[to = "/clips/{guild_id}"]
    Soundboard(u64),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

// type aliases to make life just a bit easier
pub type AppRouter = Router<AppRoute>;
pub type AppAnchor = RouterAnchor<AppRoute>;
