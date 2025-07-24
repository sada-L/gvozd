use async_trait::async_trait;
use domain::{
    models::user::{User, UserError},
    repository::user::UserRepository,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pub pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create(&self, user: &User) -> Result<(), UserError> {
        let query = r#"
            INSERT INTO users (id, username, email, password_hash)
            VALUES ($1, $2, $3, $4)
            "#;

        let res = sqlx::query(query)
            .bind(user.id)
            .bind(&user.username)
            .bind(user.email.value())
            .bind(user.password_hash.value())
            .execute(&self.pool)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(sqlx::Error::Database(db_err)) => {
                if db_err.constraint() == Some("users_email_key") {
                    Err(UserError::UserAlreadyExists)
                } else {
                    Err(UserError::DatabaseError(db_err.to_string()))
                }
            }
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }
}
