use serde_json;

#[derive(Debug, Clone)]
pub struct CoinmarketcapError {
    pub error_type: CoinmarketcapErrorType,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum CoinmarketcapErrorType {
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
