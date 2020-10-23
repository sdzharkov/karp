//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use js_sys::{Object};
use serde_json::{json, Map, Value};

extern crate karp;
use karp::karp::Karp;


wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn check_base_headers() {
    let client = Karp::new("https://graphql-weather-api.herokuapp.com/".to_string(), None::<Object>).unwrap();

    let base_headers = Map::new();
    base_headers.insert("Content-Type".to_string(), serde_json::Value::String("application/json".to_string()));
    base_headers.insert("Accept".to_string(), serde_json::Value::String("*/*".to_string()));
    
    assert_eq!(client.headers, Some(base_headers));
}
