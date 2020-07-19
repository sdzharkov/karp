use hyper::{Client, Body,body,  Method, Request, Uri, Response};
use hyper_tls::HttpsConnector;

pub struct Karp {
  pub address: String,
  client: Client<HttpsConnector<hyper::client::HttpConnector>>,
}

impl Karp {
  pub fn new(address: &str) -> Karp {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    Karp {
      address: address.to_string(),
      client,
    }
  }

  pub async fn query(&self, body: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let req = Request::builder()
        .method("POST")
        .uri(&self.address)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .body(Body::from(body.to_string()))?;
    
    let resp = self.client.request(req).await?;

    let data = read_response_body(resp).await?;

    println!("{:?}", data);

    Ok("test".to_string())
  }
}

async fn read_response_body(res: Response<Body>) -> Result<String, hyper::Error> {
  let bytes = body::to_bytes(res.into_body()).await?;
  Ok(String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8"))
}
