use rocket::{http::Status, Request};

#[catch(default)]
pub fn default(_status: Status, _request: &Request) {}
