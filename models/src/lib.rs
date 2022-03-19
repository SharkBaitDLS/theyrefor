mod user;

use serde::{Deserialize, Serialize};

pub use user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
   pub redirect_to: Option<String>,
   pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
   pub name: String,
   pub id: String,
   pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildClips {
   pub clip_names: Vec<String>,
   pub user_clip_names: Vec<String>,
   pub user_names: Vec<String>,
   pub guild_name: String,
}
