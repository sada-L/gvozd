use bcrypt::{DEFAULT_COST, hash, verify};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: Email,
    pub password_hash: PasswordHash,
}

impl User {
    pub fn new(username: String, email: String, password: String) -> Result<User, UserError> {
        Ok(Self {
            id: Uuid::new_v4(),
            username,
            email: Email::new(email)?,
            password_hash: PasswordHash::new(password)?,
        })
    }

    pub fn from(
        id: Uuid,
        username: String,
        email: String,
        password_hash: String,
    ) -> Result<User, UserError> {
        Ok(Self {
            id,
            username,
            email: Email::new(email)?,
            password_hash: PasswordHash::from_hash(password_hash),
        })
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            username: "example".to_string(),
            email: Email::default(),
            password_hash: PasswordHash::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> Result<Self, UserError> {
        if value.contains('@') {
            Ok(Self(value))
        } else {
            Err(UserError::InvalidEmail)
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for Email {
    fn default() -> Self {
        Self("example@email.com".to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(password: String) -> Result<Self, UserError> {
        if let Ok(s) = hash(password, DEFAULT_COST) {
            Ok(Self(s))
        } else {
            Err(UserError::InvalidPassword)
        }
    }

    pub fn from_hash(password: String) -> Self {
        Self(password)
    }

    pub fn verify(password: String, hash: &str) -> Result<bool, UserError> {
        if let Ok(bool) = verify(password, hash) {
            Ok(bool)
        } else {
            Err(UserError::InvalidPassword)
        }
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Default for PasswordHash {
    fn default() -> Self {
        Self("5e9f8456-af22-4561-aa1c-56d87fbe03cf".to_string())
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum UserError {
    #[error("User email is invalid")]
    InvalidEmail,

    #[error("User password is invalid")]
    InvalidPassword,

    #[error("User is not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::{DEFAULT_COST, hash, verify};

    #[test]
    fn test_password_hash_new_success() {
        let password = "secure_password123".to_string();
        let result = PasswordHash::new(password.clone());

        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(verify(password, hash.value()).unwrap());
    }

    #[test]
    fn test_password_hash_verify_success() {
        let password = "secure_password123".to_string();
        let hashed = hash(password.clone(), DEFAULT_COST).unwrap();

        let result = PasswordHash::verify(password, &hashed);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_password_hash_verify_failure() {
        let password = "secure_password123".to_string();
        let wrong_password = "wrong_password".to_string();
        let hashed = hash(password, DEFAULT_COST).unwrap();

        let result = PasswordHash::verify(wrong_password, &hashed);

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_password_hash_verify_error() {
        let password = "secure_password123".to_string();
        let invalid_hash = "invalid_hash_format";

        let result = PasswordHash::verify(password, invalid_hash);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), UserError::InvalidPassword);
    }
}
