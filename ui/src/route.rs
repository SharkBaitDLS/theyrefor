use yew_router::Routable;

#[derive(Clone, Debug, PartialEq, Eq, Routable)]
pub enum Route {
   #[at("/clips/:guild_id")]
   Soundboard { guild_id: String },
   #[at("/clips")]
   Clips,
   #[at("/servers/:guild_id")]
   Server { guild_id: String },
   #[at("/servers")]
   Servers,
   #[at("/")]
   Home,
   #[not_found]
   #[at("/page-not-found")]
   NotFound,
}
