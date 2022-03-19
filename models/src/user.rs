use serde::{Deserialize, Serialize};
#[cfg(feature = "twilight-model")]
use twilight_model::user::CurrentUser;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
   pub username: String,
   pub id: String,
   pub avatar: Option<String>,
}

#[cfg(feature = "twilight-model")]
impl From<CurrentUser> for User {
   fn from(current_user: CurrentUser) -> Self {
      User {
         username: current_user.name,
         id: current_user.id.to_string(),
         avatar: current_user
            .avatar
            .map(|avatar| format!("https://cdn.discordapp.com/avatars/{}/{}.png", current_user.id, avatar)),
      }
   }
}
