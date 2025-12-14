use rocket::{Request, http::Status};

#[catch(default)]
pub fn default(_status: Status, _request: &Request) {}
