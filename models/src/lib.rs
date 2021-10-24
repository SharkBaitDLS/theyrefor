use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Guild {
   pub name: String,
   pub id: u64,
   pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildClips {
   pub clip_names: Vec<String>,
   pub guild_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
   pub username: String,
   pub id: String,
   pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthState {
   pub redirect_to: Option<String>,
   pub token: String,
}
