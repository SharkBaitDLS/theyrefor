pub mod auth;
pub mod clips;
pub mod guilds;
pub mod user;

use rocket::http::Status;

pub type ApiError = (Status, String);
pub type ApiResponse<T> = Result<T, (Status, String)>;
