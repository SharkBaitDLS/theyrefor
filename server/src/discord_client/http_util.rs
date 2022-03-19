use reqwest::{Error, Response, StatusCode};
use rocket::http::Status;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

use crate::api::{ApiError, ApiResponse};

pub fn internal_server_error<T: Debug>(err: T) -> ApiError {
   error!("{:?}", err);
   (Status::InternalServerError, String::new())
}

pub async fn check_ok(body: Result<Response, Error>) -> ApiResponse<()> {
   match body {
      Ok(body) if body.status() == StatusCode::OK => Ok(()),
      Ok(body) => {
         let status = body.status();
         let text = body.text().await.map_err(internal_server_error)?;
         error!("Failed request: {}", text);
         Err((Status::new(status.into()), String::new()))
      }
      Err(err) => Err(internal_server_error(err)),
   }
}

pub async fn deserialize<T: DeserializeOwned>(body: Result<Response, Error>) -> ApiResponse<T> {
   match body {
      Ok(body) if body.status() == StatusCode::OK => body.json().await.map_err(internal_server_error),
      Ok(body) => {
         let status = body.status();
         let text = body.text().await.map_err(internal_server_error)?;
         error!("Failed request: {}", text);
         Err((Status::new(status.into()), String::new()))
      }
      Err(err) => Err(internal_server_error(err)),
   }
}
