use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use reqwest::{Client};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub struct Karp {
  address: String,
}

#[wasm_bindgen]
impl Karp {
  pub fn new(address: String) -> Karp {

    Karp {
      address: address,
    }
  }

  pub async fn query(self, body: JsValue) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    opts.body(Some(&body));

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
}
