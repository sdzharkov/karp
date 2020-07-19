extern crate karp;
use karp::Karp;
use hyper::{Client, Body, Method, Request, Uri, Response};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = "http://localhost:3000/graphql";
    let karp = Karp::new(addr);

    let body = "{ \"query\": \"query test { contractor(id: 22) { firstName } }\" }";

    let resp = karp.query(body).await?;
    // let (parts, body) = resp.into_parts();

    Ok(())
}
