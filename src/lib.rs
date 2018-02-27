#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod model;
pub use model::*;

const API_URL: &str = "https://api.coinmarketcap.com/v1";

pub fn all() -> Vec<CoinCap> {
    let client = reqwest::Client::new();
    let mut response = reqwest::get("https://api.coinmarketcap.com/v1/ticker?limit=0").expect("/v1/ticker to respond correctly");
    let body = response.text().expect("json response to have text");

    serde_json::from_str(&body).expect("json to deserialise")
}
