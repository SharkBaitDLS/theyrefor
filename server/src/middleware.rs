use reqwest::{header::HeaderValue, Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use task_local_extensions::Extensions;

pub struct UserAgentMiddleware;

#[async_trait::async_trait]
impl Middleware for UserAgentMiddleware {
   async fn handle(&self, mut req: Request, extensions: &mut Extensions, next: Next<'_>) -> Result<Response> {
      let headers = req.headers_mut();
      headers.insert(
         reqwest::header::USER_AGENT,
         HeaderValue::from_static("theyrefor-server/0.1"),
      );
      next.run(req, extensions).await
   }
}
