use serde_json;
use reqwest;

#[derive(Debug, Clone)]
pub struct CoinmarketcapError {
    pub error_type: CoinmarketcapErrorType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum CoinmarketcapErrorType {
    RequestError,
    ParseError,
}

impl From<serde_json::Error> for CoinmarketcapError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            error_type: CoinmarketcapErrorType::ParseError,
            message: error.to_string(),
        }
    }
}

impl From<reqwest::Error> for CoinmarketcapError {
    fn from(error: reqwest::Error) -> Self {
        Self {
            error_type: CoinmarketcapErrorType::RequestError,
            message: error.to_string(),
        }
    }
}
