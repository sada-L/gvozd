use chrono::{Duration, Utc};
use domain::models::token::{Token, TokenClaims, TokenError};
use domain::models::user::User;
use domain::repository::token::TokenService;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};

static ONE_HOUR: i64 = 60 * 60;

pub struct TokenServiceImpl {
    secret: String,
    exp: i64,
}

impl TokenServiceImpl {
    pub fn new(secret: String, exp: i64) -> Self {
        Self { secret, exp }
    }
}

impl TokenService for TokenServiceImpl {
    fn generate_token(&self, user: &User) -> Result<Token, TokenError> {
        let access_token = generate_access_token(user, &self.secret)?;
        let refresh_token = generate_refresh_token(user, &self.secret, self.exp)?;

        Ok(Token {
            access_token,
            refresh_token,
        })
    }

    fn verify_token(&self, token: &str) -> Result<TokenClaims, TokenError> {
        let secret = &self.secret;
        decode(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|_| TokenError::InvalidToken)
    }
}

fn generate_access_token(user: &User, secret: &str) -> Result<String, TokenError> {
    let exp = Utc::now()
        .checked_add_signed(Duration::hours(ONE_HOUR))
        .unwrap()
        .timestamp();

    let access_claims = TokenClaims {
        sub: user.id.to_string(),
        opt: Some(user.email.value().to_string()),
        exp,
    };

    encode(
        &Header::default(),
        &access_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| TokenError::InvalidClaims)
}

fn generate_refresh_token(user: &User, secret: &str, exp: i64) -> Result<String, TokenError> {
    let exp = Utc::now()
        .checked_add_signed(Duration::days(exp))
        .unwrap()
        .timestamp();

    let refresh_claims = TokenClaims {
        sub: user.id.to_string(),
        opt: None,
        exp,
    };

    encode(
        &Header::default(),
        &refresh_claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| TokenError::InvalidClaims)
}
