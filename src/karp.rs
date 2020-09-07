use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};

macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

#[wasm_bindgen]
pub struct Karp {
  address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
  query: String,
}

#[wasm_bindgen]
impl Karp {
  pub fn new(address: String) -> Karp {

    Karp {
      address: address,
    }
  }

  pub async fn query(self, body: String) -> Result<JsValue, JsValue> {
    let request_body = self.create_body(&body).unwrap();

    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&request_body));

    let request = Request::new_with_str_and_init(&self.address, &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    request.headers().set("Accept", "*/*")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
    // Use serde to parse the JSON into a struct.
    // let branch_info = json.into_serde().unwrap();

    // Send the `Branch` struct back to JS as an `Object`.
    // Ok(JsValue::from_serde(&branch_info).unwrap())
  }

  /// Creates the body used in the request
  /// 
  /// # Arguments
  /// * `body` - The string literal passed from the frotend
  /// 
  fn create_body(&self, body: &String) -> Result<JsValue, serde_json::Error> {
    // @TODO: transfer ownership without clone
    let query = Query {
      query: body.clone(),
    };

    let t = serde_json::to_string(&query)?;

    Ok(JsValue::from_str(&t))
  }
}
