#![allow(dead_code)]
#![allow(unused_variables)]

use std::str::FromStr;
use std::fmt::Display;
use serde::de::{self, Deserialize, Deserializer};

// Any value that is present is considered Some value, including null.
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    match Deserialize::deserialize(deserializer) {
        Ok(s) => Ok(Some(T::from_str(s).map_err(de::Error::custom)?)),
        Err(e) => Ok(None)
    }
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinCap {
    pub id: String,
    pub name: String,
    pub symbol: String,
    #[serde(deserialize_with = "from_str")]
    pub rank: u32,
    #[serde(deserialize_with = "deserialize_some")]
    pub price_usd: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub price_btc: Option<f64>,
    #[serde(rename = "24h_volume_usd")]
    #[serde(deserialize_with = "deserialize_some")]
    pub volume_usd_24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub market_cap_usd: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub available_supply: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub total_supply: Option<f64>,
    pub max_supply: Option<String>,
    #[serde(deserialize_with = "deserialize_some")]
    pub percent_change_1h: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub percent_change_24h: Option<f64>,
    #[serde(deserialize_with = "deserialize_some")]
    pub percent_change_7d: Option<f64>,
    pub last_updated: Option<String>,
}
