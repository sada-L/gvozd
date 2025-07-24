use domain::models::user::{self, User, UserError};

use crate::errors::AppError;

#[derive(Debug, Default)]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl TryInto<User> for UserDTO {
    type Error = user::UserError;

    fn try_into(self) -> Result<User, UserError> {
        User::new(self.username, self.email, self.password)
    }
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        Self {
            username: value.username,
            email: value.email.value().to_string(),
            password: value.password_hash.value().to_string(),
        }
    }
}

impl From<UserError> for AppError {
    fn from(value: UserError) -> Self {
        match value {
            UserError::DatabaseError(msg) => AppError::DatabaseError(msg),
            UserError::UserNotFound => AppError::NotFound(UserError::UserNotFound.to_string()),
            UserError::InvalidEmail => AppError::BadRequest(UserError::InvalidEmail.to_string()),
            UserError::InvalidPassword => {
                AppError::BadRequest(UserError::InvalidPassword.to_string())
            }
            UserError::UserAlreadyExists => {
                AppError::AlreadyExists(UserError::UserAlreadyExists.to_string())
            }
        }
    }
}
