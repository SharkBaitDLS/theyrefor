use rocket::{
   figment::Source,
   fs::NamedFile,
   http::{
      uri::{fmt::Path, Segments},
      Method,
   },
   route::{Handler, Outcome},
   Data, Request, Route,
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct SPAServer {
   root: PathBuf,
}

impl SPAServer {
   pub fn from<P: AsRef<std::path::Path>>(path: P) -> Self {
      let mut root = PathBuf::new();
      root.push(path);
      Self { root }
   }
}

impl From<SPAServer> for Vec<Route> {
   fn from(server: SPAServer) -> Self {
      let source = Source::File(server.root.clone());
      let mut route = Route::new(Method::Get, "/<_..>", server);
      route.name = Some(format!("FileServer: {source}").into());
      vec![route]
   }
}

#[async_trait::async_trait]
impl Handler for SPAServer {
   async fn handle<'r>(&self, req: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
      let path = req
         .segments::<Segments<'_, Path>>(0..)
         .ok()
         .and_then(|segments| segments.last())
         .map(|path| self.root.join(path));

      match path {
         Some(p) if p.exists() => Outcome::try_from(req, NamedFile::open(p).await),
         _ => Outcome::try_from(req, NamedFile::open(self.root.join("index.html")).await),
      }
   }
}
