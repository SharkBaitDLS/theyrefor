use base64::engine::{general_purpose::URL_SAFE, Engine};
use http::StatusCode;
use log::error;
use reqwasm::{
   http::{Request, Response},
   Error,
};
use serde::de::DeserializeOwned;
use url::Url;
use wasm_bindgen::JsValue;

use theyrefor_models::AuthState;

static STATE_PARAM: &str = "state";

#[derive(Debug)]
pub enum ClientError {
   Client,
   #[allow(dead_code)] // used for display in Debug
   Status(u16),
}
impl From<Error> for ClientError {
   fn from(_: Error) -> Self {
      ClientError::Client
   }
}

pub async fn get_with_auth<T>(uri: &str) -> Result<Option<T>, ClientError>
where
   T: DeserializeOwned,
{
   match Request::get(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) if response.status() == StatusCode::OK => response.json().await.map_err(|err| err.into()),
      Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(None),
      Ok(response) => {
         error!("Unexpected response: {:?}", response.status());
         Err(ClientError::Status(response.status()))
      }
      Err(err) => {
         error!("{:?}", err);
         Err(err.into())
      }
   }
}

pub async fn put_with_auth<V: Into<JsValue>>(uri: &str, body: V) -> Result<Option<()>, ClientError> {
   match Request::put(uri).body(body).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) if response.status() == StatusCode::OK => Ok(Some(())),
      Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(None),
      Ok(response) => {
         error!("Unexpected response: {:?}", response.status());
         Err(ClientError::Status(response.status()))
      }
      Err(err) => {
         error!("{:?}", err);
         Err(err.into())
      }
   }
}

pub async fn post_with_auth(uri: &str) -> Result<Option<()>, ClientError> {
   match Request::post(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) if response.status() == StatusCode::OK => Ok(Some(())),
      Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(None),
      Ok(response) => {
         error!("Unexpected response: {:?}", response.status());
         Err(ClientError::Status(response.status()))
      }
      Err(err) => {
         error!("{:?}", err);
         Err(err.into())
      }
   }
}

pub async fn delete_with_auth(uri: &str) -> Result<Option<()>, ClientError> {
   match Request::delete(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) if response.status() == StatusCode::OK => Ok(Some(())),
      Ok(response) if response.status() == StatusCode::NOT_FOUND => Ok(None),
      Ok(response) => {
         error!("Unexpected response: {:?}", response.status());
         Err(ClientError::Status(response.status()))
      }
      Err(err) => {
         error!("{:?}", err);
         Err(err.into())
      }
   }
}

async fn update_redirect<T>(response: Response) -> Result<Option<T>, ClientError> {
   let location = web_sys::window().unwrap().location();

   let text = response.text().await;
   let maybe_url = text.as_ref().ok().and_then(|text| Url::parse(text).ok());
   if maybe_url.is_none() {
      error!("Could not parse redirect uri: {}", text.unwrap_or_default());
      return Ok(None);
   }
   let url = maybe_url.unwrap();

   let state_param = url.query_pairs().find(|pair| pair.0 == STATE_PARAM).unwrap().1;
   let mut auth_state: AuthState = bincode::deserialize(&URL_SAFE.decode(&*state_param).unwrap()).unwrap();
   auth_state.redirect_to = location.href().ok();

   let remainder = url.query_pairs().filter(|pair| pair.0 != STATE_PARAM);
   let mut url = url.clone();
   url.set_query(None);
   url.query_pairs_mut()
      .extend_pairs(remainder)
      .append_pair(STATE_PARAM, &URL_SAFE.encode(bincode::serialize(&auth_state).unwrap()));

   location
      .set_href(url.as_str())
      .map(|_| None)
      .map_err(|_| ClientError::Client)
}
