use application::dto::token::TokenDTO;
use serde::Serialize;

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
}

impl From<TokenDTO> for AuthResponse {
    fn from(value: TokenDTO) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}
