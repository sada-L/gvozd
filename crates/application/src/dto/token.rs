use domain::models::token::{Token, TokenError};

use crate::errors::AppError;

#[derive(Debug, Default)]
pub struct TokenDTO {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<Token> for TokenDTO {
    fn from(value: Token) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}

impl From<TokenError> for AppError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::InvalidToken => AppError::BadRequest(TokenError::InvalidToken.to_string()),
            TokenError::InvalidClaims => {
                AppError::BadRequest(TokenError::InvalidClaims.to_string())
            }
            TokenError::InvalidSecret => {
                AppError::BadRequest(TokenError::InvalidSecret.to_string())
            }
        }
    }
}
