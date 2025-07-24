use async_trait::async_trait;

use crate::models::user::{User, UserError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), UserError>;
}
