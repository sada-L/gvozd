use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub opt: Option<String>,
    pub exp: i64,
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error("Invalid claims")]
    InvalidClaims,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Invalid secret")]
    InvalidSecret,
}
