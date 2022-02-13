use log::error;
use reqwasm::{
   http::{Request, Response},
   Error,
};
use serde::de::DeserializeOwned;
use url::Url;
use yew::services::fetch::StatusCode;

use theyrefor_models::AuthState;

static STATE_PARAM: &str = "state";

pub async fn get_with_auth<T>(uri: &str) -> Result<Option<T>, Error>
where
   T: DeserializeOwned,
{
   match Request::get(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) if response.status() == StatusCode::OK => response.json().await,
      Ok(response) => {
         error!("Unexpected response: {:?}", response.status());
         Ok(None)
      }
      Err(err) => {
         error!("{:?}", err);
         Err(err)
      }
   }
}

pub async fn post_with_auth(uri: &str) -> Result<Option<()>, Error> {
   match Request::post(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) if response.status() == StatusCode::OK => Ok(Some(())),
      Ok(response) => {
         error!("Unexpected response: {:?}", response.status());
         Ok(None)
      }
      Err(err) => {
         error!("{:?}", err);
         Err(err)
      }
   }
}

async fn update_redirect<T>(response: Response) -> Result<Option<T>, Error> {
   let location = web_sys::window().unwrap().location();

   let url = response
      .text()
      .await
      .ok()
      .and_then(|text| Url::parse(&text).ok())
      .unwrap();
   let state_param = url.query_pairs().find(|pair| pair.0 == STATE_PARAM).unwrap().1;
   let mut auth_state: AuthState = bincode::deserialize(&base64::decode(&*state_param).unwrap()).unwrap();
   auth_state.redirect_to = location.href().ok();

   let remainder = url.query_pairs().filter(|pair| pair.0 != STATE_PARAM);
   let mut url = url.clone();
   url.set_query(None);
   url.query_pairs_mut()
      .extend_pairs(remainder)
      .append_pair(STATE_PARAM, &base64::encode(&bincode::serialize(&auth_state).unwrap()));

   location
      .set_href(url.as_str())
      .map(|_| None)
      .map_err(|err| Error::JsError(err.try_into().unwrap()))
}
