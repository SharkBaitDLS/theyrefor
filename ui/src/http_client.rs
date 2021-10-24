use anyhow::anyhow;
use log::error;
use reqwasm::{
   http::{Request, Response},
   Error,
};
use serde::de::DeserializeOwned;
use theyrefor_models::AuthState;
use url::Url;
use yew::services::fetch::StatusCode;

static STATE_PARAM: &str = "state";

pub async fn get_with_auth<T>(uri: &str) -> Result<Option<T>, Error>
where
   T: DeserializeOwned,
{
   match Request::get(uri).send().await {
      Ok(response) if response.status() == StatusCode::UNAUTHORIZED => update_redirect(response).await,
      Ok(response) => response.json().await,
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
      .map_err(|_| Error::Other(anyhow!("Failed to update HREF")))
}
