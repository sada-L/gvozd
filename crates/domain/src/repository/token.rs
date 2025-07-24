use crate::models::{
    token::{Token, TokenClaims, TokenError},
    user::User,
};

pub trait TokenService: Send + Sync {
    fn generate_token(&self, user: &User) -> Result<Token, TokenError>;
    fn verify_token(&self, token: &str) -> Result<TokenClaims, TokenError>;
}
