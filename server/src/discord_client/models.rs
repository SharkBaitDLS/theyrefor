use serde::Deserialize;
use theyrefor_models::Guild;

#[derive(Deserialize)]
pub struct DiscordAuthResponse {
   pub access_token: String,
   pub expires_in: u64,
   pub refresh_token: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct DiscordGuild {
   pub name: String,
   pub id: String,
   pub icon: Option<String>,
   pub owner: bool,
}

#[derive(Debug, Deserialize)]
pub struct DiscordGuildRoles {
   pub roles: Vec<String>,
}

impl From<DiscordGuild> for Guild {
   fn from(val: DiscordGuild) -> Self {
      Guild {
         name: val.name.clone(),
         id: val.id.clone(),
         icon: val
            .icon
            .as_ref()
            .map(|icon| format!("https://cdn.discordapp.com/icons/{}/{}.png", val.id, icon)),
      }
   }
}
