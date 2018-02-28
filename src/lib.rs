#![allow(dead_code)]
#![allow(unused_variables)]

extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde;

mod model;
pub use model::*;

mod error;
pub use error::*;

const API_URL: &str = "https://api.coinmarketcap.com/v1";

pub fn all(limit: u32) -> Result<Vec<CoinCap>, CoinmarketcapError> {
    let client = reqwest::Client::new();
    let request_url = &format!("{}/ticker?limit={}", API_URL, limit);

    let mut response = reqwest::get(request_url).expect("/v1/ticker to respond correctly");
    let body = response.text().expect("json response to have text");

    Ok(serde_json::from_str(&body)?)
}
