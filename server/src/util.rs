use std::fmt::Debug;

use reqwest::{Error, Response, StatusCode};
use rocket::{http::Status, serde::DeserializeOwned};

use crate::util;

pub fn internal_server_error<T: Debug>(err: T) -> (Status, String) {
   error!("{:?}", err);
   (Status::InternalServerError, String::new())
}

pub async fn check_ok(body: Result<Response, Error>) -> Result<(), (Status, String)> {
   match body {
      Ok(body) if body.status() == StatusCode::OK => Ok(()),
      Ok(body) => {
         let status = body.status();
         let text = body.text().await.map_err(util::internal_server_error)?;
         error!("Failed request: {}", text);
         Err((Status::new(status.into()), String::new()))
      }
      Err(err) => Err(util::internal_server_error(err)),
   }
}

pub async fn deserialize<T: DeserializeOwned>(body: Result<Response, Error>) -> Result<T, (Status, String)> {
   match body {
      Ok(body) if body.status() == StatusCode::OK => body.json().await.map_err(util::internal_server_error),
      Ok(body) => {
         let status = body.status();
         let text = body.text().await.map_err(util::internal_server_error)?;
         error!("Failed request: {}", text);
         Err((Status::new(status.into()), String::new()))
      }
      Err(err) => Err(util::internal_server_error(err)),
   }
}
