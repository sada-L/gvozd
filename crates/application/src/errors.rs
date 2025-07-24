use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("already exists: {0}")]
    AlreadyExists(String),

    #[error("database error: {0}")]
    DatabaseError(String),
}
