use serde::Deserialize;
use theyrefor_models::Guild;
use twilight_model::{
   guild::Member,
   user::{CurrentUserGuild, User},
};

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
pub struct DiscordGuildMember {
   pub user: DiscordGuildUser,
   pub roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DiscordGuildUser {
   pub username: String,
}

impl From<DiscordGuild> for Guild {
   fn from(val: DiscordGuild) -> Self {
      Guild {
         name: val.name,
         id: val.id,
         icon: val.icon,
      }
   }
}

impl From<CurrentUserGuild> for DiscordGuild {
   fn from(current_user_guild: CurrentUserGuild) -> Self {
      DiscordGuild {
         name: current_user_guild.name,
         id: current_user_guild.id.to_string(),
         icon: current_user_guild.icon.map(|icon| {
            format!(
               "https://cdn.discordapp.com/icons/{}/{}.png",
               current_user_guild.id, icon
            )
         }),
         owner: current_user_guild.owner,
      }
   }
}

impl From<User> for DiscordGuildUser {
   fn from(user: User) -> Self {
      DiscordGuildUser { username: user.name }
   }
}

impl From<Member> for DiscordGuildMember {
   fn from(member: Member) -> Self {
      DiscordGuildMember {
         user: member.user.into(),
         roles: member.roles.into_iter().map(|role| role.to_string()).collect(),
      }
   }
}
