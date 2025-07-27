use async_trait::async_trait;
use uuid::Uuid;

use crate::models::user::{User, UserError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), UserError>;
    async fn update(&self, user: &User) -> Result<(), UserError>;
    async fn delete(&self, id: Uuid) -> Result<(), UserError>;
    async fn find_by_id(&self, id: Uuid) -> Option<User>;
    async fn find_by_email(&self, email: &str) -> Option<User>;
}
