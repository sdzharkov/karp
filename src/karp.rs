use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[allow(unused_macros)]
macro_rules! log {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into());
  }
}

#[wasm_bindgen]
pub struct Karp {
  address: String,
  headers: Option<Map<String, Value>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
  query: String,
}

#[wasm_bindgen]
impl Karp {
  pub fn new(address: String, config: Option<js_sys::Object>) -> Result<Karp, JsValue> {
    let mut karp = Karp { address, headers: None };
    let mut karp_config: Map<String, serde_json::Value> = JsValue::from(config).into_serde().unwrap();


    if karp_config.contains_key("headers") {
      if let Some(headers) = karp_config.get_mut("headers") {
        if let Some(header_obj) = headers.as_object_mut() {
          header_obj.insert("Content-Type".to_string(), serde_json::Value::String("application/json".to_string()));
          header_obj.insert("Accept".to_string(), serde_json::Value::String("*/*".to_string()));

          // @TODO: I use mem::take to avoid cloneing. I no longer nead the header obj
          // from above. Is this a bad practice?
          karp.headers = Some(std::mem::take(header_obj));
        }
      }
    }

    return Ok(karp);
  }

  pub fn set_headers(&mut self, headers: &JsValue) {
    let mut original_headers: Map<String, serde_json::Value> = match headers.into_serde() {
      Ok(t) => t,
      Err(_e) => Map::new()
    };
    original_headers.insert("Content-Type".to_string(), serde_json::Value::String("application/json".to_string()));
    original_headers.insert("Accept".to_string(), serde_json::Value::String("*/*".to_string()));

    self.headers = Some(original_headers);
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
      query: body.to_string(),
    };

    let t = serde_json::to_string(&query)?;

    Ok(JsValue::from_str(&t))
  }
}
