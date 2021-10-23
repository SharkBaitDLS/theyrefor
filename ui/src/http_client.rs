use log::error;
use reqwasm::{http::Request, Error};
use serde::de::DeserializeOwned;
use yew::services::fetch::StatusCode;

pub async fn get<T>(uri: &str) -> Result<Option<T>, Error>
where
   T: DeserializeOwned,
{
   match Request::get(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => {
         let redirect = response.text().await.unwrap();
         web_sys::window().map(|window| window.location().set_href(&redirect));
         Ok(None)
      }
      Ok(response) => response.json().await,
      Err(err) => {
         error!("{:?}", err);
         Err(err)
      }
   }
}
