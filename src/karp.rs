use hyper::{Body,body,  Method, Request, Uri, Response};
use hyper_tls::HttpsConnector;
use wasm_bindgen::prelude::*;
use reqwest::{Client};

#[cfg(target_arch = "wasm32")]
impl From<reqwest::error::Error> for wasm_bindgen::JsValue {
    fn from(err: reqwest::error::Error) -> wasm_bindgen::JsValue {
        js_sys::Error::from(err).into()
    }
}

#[cfg(target_arch = "wasm32")]
impl From<reqwest::error::Error> for js_sys::Error {
    fn from(err: reqwest::error::Error) -> js_sys::Error {
        js_sys::Error::new(&format!("{}", err))
    }
}

#[wasm_bindgen]
pub struct Karp {
  address: String,
  client: Client,
}

#[wasm_bindgen]
impl Karp {
  pub fn new(address: String) -> Karp {
    let https = HttpsConnector::new();
    let client = Client::new();

    Karp {
      address: address,
      client,
    }
  }

  pub async fn query(&self, body: &str) -> Result<JsValue, JsValue> {
    // let req = Request::builder()
    //     .method("POST")
    //     .uri(&self.address)
    //     .header("Content-Type", "application/json")
    //     .header("Accept", "application/json")
    //     .body(Body::from(body.to_string()))?;
    
    let resp = self.client
      .post(&self.address)
      .body(body)
      .send()
      .await?;

    let data = read_response_body(resp).await?;

    println!("{:?}", data);

    Ok(JsValue::from_str(data.as_str()))
  }
}

async fn read_response_body(res: Response<Body>) -> Result<String, hyper::Error> {
  let bytes = body::to_bytes(res.into_body()).await?;
  Ok(String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8"))
}

// Investigate this - using web api's directly:
// https://github.com/rustwasm/wasm-bindgen/blob/master/examples/fetch/src/lib.rs
