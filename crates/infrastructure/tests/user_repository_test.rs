#[cfg(test)]
mod test {
    use domain::{
        models::user::{Email, PasswordHash, User, UserError},
        repository::user::UserRepository,
    };
    use sqlx::{PgPool, Row};
    use testcontainers::{ContainerAsync, runners::AsyncRunner};
    use testcontainers_modules::postgres::Postgres;
    use uuid::Uuid;

    use infrastructure::{db, repository::user::UserRepositoryImpl};

    async fn create_pool() -> (PgPool, ContainerAsync<Postgres>) {
        let node = Postgres::default()
            .start()
            .await
            .expect("Error start container");

        let port = node.get_host_port_ipv4(5432).await.unwrap();
        let connection_str = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

        let pool = db::init_db_pool(&connection_str, 5)
            .await
            .expect("Error connect to database");

        db::run_migration(&pool, "../../migrations").await;

        (pool, node)
    }

    #[tokio::test]
    async fn test_create_user() {
        let (pool, _node) = create_pool().await;

        let sut = UserRepositoryImpl::new(&pool);

        let email = Email::new("duplicate@example.com".to_string()).unwrap();

        let user1 = User {
            id: Uuid::new_v4(),
            username: "user1".to_string(),
            email: email.clone(),
            password_hash: PasswordHash::default(),
        };

        let user2 = User {
            id: Uuid::new_v4(),
            username: "user2".to_string(),
            email,
            password_hash: PasswordHash::default(),
        };

        let res1 = sut.create(&user1).await;

        let row = sqlx::query("SELECT username FROM users WHERE id = $1")
            .bind(user1.id)
            .fetch_one(&pool)
            .await
            .expect("User not found in DB");

        let username: String = row.get("username");

        assert!(res1.is_ok());
        assert_eq!(username, "user1");

        let res2 = sut.create(&user2).await;

        assert!(res2.is_err());
        assert!(matches!(res2, Err(UserError::UserAlreadyExists)));
    }
}
