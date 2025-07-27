use async_trait::async_trait;
use domain::{
    models::user::{User, UserError},
    repository::user::UserRepository,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;

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
            INSERT INTO users (id, username, email, password_hash, secret)
            VALUES ($1, $2, $3, $4, $5)
            "#;

        let res = sqlx::query(query)
            .bind(user.id)
            .bind(&user.username)
            .bind(user.email.value())
            .bind(user.password_hash.value())
            .bind(&user.secret)
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

    async fn update(&self, user: &User) -> Result<(), UserError> {
        let query = r#"
            UPDATE users
            SET username = $1, email = $2, secret = $3 where id = $4
        "#;

        let res = sqlx::query(query)
            .bind(&user.username)
            .bind(user.email.value())
            .bind(&user.secret)
            .bind(user.id)
            .execute(&self.pool)
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), UserError> {
        let query = r#"
            DELETE FROM users WHERE id = $1
        "#;

        let res = sqlx::query(query).bind(id).execute(&self.pool).await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(UserError::DatabaseError(e.to_string())),
        }
    }

    async fn find_by_id(&self, id: uuid::Uuid) -> Option<User> {
        let query = r#"
            SELECT id, username, email, password_hash, secret FROM users WHERE id = $1
        "#;

        let res = sqlx::query(query).bind(id).fetch_optional(&self.pool).await;

        match res {
            Ok(Some(row)) => User::from(
                id,
                row.get("username"),
                row.get("email"),
                row.get("password_hash"),
                row.try_get("secret").ok(),
            )
            .ok(),
            Ok(None) => None,
            Err(_) => None,
        }
    }

    async fn find_by_email(&self, email: &str) -> Option<User> {
        let query = r#"
            SELECT id, username, email, password_hash, secret FROM users WHERE email = $1
        "#;

        let res = sqlx::query(query)
            .bind(email)
            .fetch_optional(&self.pool)
            .await;

        match res {
            Ok(Some(row)) => User::from(
                row.get("id"),
                row.get("username"),
                row.get("email"),
                row.get("password_hash"),
                row.try_get("secret").ok(),
            )
            .ok(),
            Ok(None) => None,
            Err(_) => None,
        }
    }
}
