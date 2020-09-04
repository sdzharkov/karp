use wasm_bindgen::prelude::*;
use reqwest::{Client};

#[wasm_bindgen]
pub struct Karp {
  address: String,
  client: Client,
}

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    let res = reqwest::Client::new()
        .get("https://api.github.com/repos/rustwasm/wasm-bindgen/branches/master")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await?;

    let text = res.text().await?;
    let branch_info: Branch = serde_json::from_str(&text).unwrap();

    Ok(JsValue::from_serde(&branch_info).unwrap())
}

#[wasm_bindgen]
impl Karp {
  pub fn new(address: String) -> Karp {
    let client = Client::new();

    Karp {
      address: address,
      client,
    }
  }

  pub async fn query(&self, body: &str) -> Result<JsValue, JsValue> {
    let resp = self.client
      .post(&self.address)
      .body(body)
      .header("Content-Type", "application/json")
      .header("Accept", "application/json")
      .send()
      .await?;

    println!("{:?}", data);

    Ok(JsValue::from_str(data.as_str()))
  }
}

// Investigate this - using web api's directly:
// https://github.com/rustwasm/wasm-bindgen/blob/master/examples/fetch/src/lib.rs
