use rocket::{
   fairing::{Fairing, Info, Kind},
   http::Header,
   Request, Response,
};

use crate::Env;

pub struct Cors;

#[async_trait::async_trait]
impl Fairing for Cors {
   fn info(&self) -> Info {
      Info {
         name: "Add CORS headers to responses",
         kind: Kind::Response,
      }
   }

   async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
      let env = request.rocket().state::<Env>().unwrap();
      response.set_header(Header::new("Access-Control-Allow-Origin", &env.base_uri));
      response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET"));
   }
}
