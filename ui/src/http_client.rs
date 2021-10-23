use anyhow::anyhow;
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
         // TODO: extract token and rewrite redirect location in state
         let base_redirect = response.text().await.unwrap();
         let location = web_sys::window().unwrap().location();

         location
            .set_href(&base_redirect)
            .map(|_| None)
            .map_err(|_| Error::Other(anyhow!("")))
      }
      Ok(response) => response.json().await,
      Err(err) => {
         error!("{:?}", err);
         Err(err)
      }
   }
}
