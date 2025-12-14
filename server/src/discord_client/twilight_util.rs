use err_into::MapInto;
use itertools::Itertools;
use rocket::http::Status;
use serde::de::DeserializeOwned;
use std::marker::Unpin;
use twilight_http::{Error, Response, response::marker::ListBody};

use crate::api::ApiResponse;

use super::http_util::internal_server_error;

pub async fn marshal<T: Into<O> + DeserializeOwned + Unpin, O>(result: Result<Response<T>, Error>) -> ApiResponse<O> {
   match result {
      Ok(body) if body.status().is_success() => body.model().await.map_into().map_err(internal_server_error),
      Ok(body) => {
         let status = body.status();
         let text = body.text().await.map_err(internal_server_error)?;
         error!("Failed request: {}", text);
         Err((Status::new(status.get()), String::new()))
      }
      Err(err) => Err(internal_server_error(err)),
   }
}

pub async fn marshal_list<T: Into<O> + DeserializeOwned + Unpin, O>(
   result: Result<Response<ListBody<T>>, Error>,
) -> ApiResponse<Vec<O>> {
   match result {
      Ok(body) if body.status().is_success() => body
         .models()
         .await
         .map(|models| models.into_iter().map_into().collect())
         .map_err(internal_server_error),
      Ok(body) => {
         let status = body.status();
         let text = body.text().await.map_err(internal_server_error)?;
         error!("Failed request: {}", text);
         Err((Status::new(status.get()), String::new()))
      }
      Err(err) => Err(internal_server_error(err)),
   }
}
