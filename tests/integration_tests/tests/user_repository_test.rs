#[cfg(test)]
mod test {
    use domain::{
        models::user::{User, UserError},
        repository::user::UserRepository,
    };
    use infrastructure::repository::user::UserRepositoryImpl;
    use integration_tests::TestContext;
    use sqlx::Row;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_create_user_success() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);
        let user = User::default();

        let res1 = sut.create(&user).await;

        let row = sqlx::query("SELECT username FROM users WHERE id = $1")
            .bind(user.id)
            .fetch_one(&ctx.pool)
            .await
            .expect("User not found in DB");

        let username: String = row.get("username");
        assert!(res1.is_ok());
        assert_eq!(username, user.username);
    }

    #[tokio::test]
    async fn test_create_user_email_already_exists() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);
        let user1 = User::default();
        let mut user2 = user1.clone();
        user2.id = Uuid::new_v4();

        let res1 = sut.create(&user1).await;
        let res2 = sut.create(&user2).await;

        assert!(res1.is_ok());
        assert!(res2.is_err());
        assert!(matches!(res2, Err(UserError::UserAlreadyExists)));
    }

    #[tokio::test]
    async fn test_update_user_success() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);
        let old_user = User::default();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, secret)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(old_user.id)
        .bind(&old_user.username)
        .bind(old_user.email.value())
        .bind(old_user.password_hash.value())
        .bind(&old_user.secret)
        .execute(&ctx.pool)
        .await
        .expect("Failed to insert test user");

        let mut new_user = old_user.clone();
        new_user.username = "new_username".to_string();

        let res = sut.update(&new_user).await;

        let row = sqlx::query("SELECT username FROM users WHERE id = $1")
            .bind(old_user.id)
            .fetch_one(&ctx.pool)
            .await
            .expect("User not found in DB");

        let username: String = row.get("username");
        assert!(res.is_ok());
        assert_eq!(username, new_user.username);
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);
        let test_user = User::default();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, secret)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(test_user.id)
        .bind(&test_user.username)
        .bind(test_user.email.value())
        .bind(test_user.password_hash.value())
        .bind(&test_user.secret)
        .execute(&ctx.pool)
        .await
        .expect("Failed to insert test user");

        let res = sut.delete(test_user.id).await;

        assert!(res.is_ok());
        let res = sqlx::query("SELECT username FROM users WHERE id = $1")
            .bind(test_user.id)
            .fetch_one(&ctx.pool)
            .await;

        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_find_by_email_success() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);
        let test_user = User::default();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, secret)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(test_user.id)
        .bind(&test_user.username)
        .bind(test_user.email.value())
        .bind(test_user.password_hash.value())
        .bind(&test_user.secret)
        .execute(&ctx.pool)
        .await
        .expect("Failed to insert test user");

        let res = sut.find_by_email(test_user.email.value()).await;

        assert!(res.is_some());
        let user = res.unwrap();
        assert_eq!(user.email, test_user.email);
        assert_eq!(user.username, test_user.username);
        assert_eq!(user.password_hash, test_user.password_hash);
        assert_eq!(user.secret, test_user.secret);
    }

    #[tokio::test]
    async fn test_find_by_email_user_not_exists() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);

        let res = sut.find_by_email("noexistsemail@mail.com").await;

        assert!(res.is_none());
    }

    #[tokio::test]
    async fn test_find_by_id_success() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);
        let test_user = User::default();

        sqlx::query(
            r#"
            INSERT INTO users (id, username, email, password_hash, secret)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(test_user.id)
        .bind(&test_user.username)
        .bind(test_user.email.value())
        .bind(test_user.password_hash.value())
        .bind(&test_user.secret)
        .execute(&ctx.pool)
        .await
        .expect("Failed to insert test user");

        let res = sut.find_by_id(test_user.id).await;

        assert!(res.is_some());
        let user = res.unwrap();
        assert_eq!(user.email, test_user.email);
        assert_eq!(user.username, test_user.username);
        assert_eq!(user.password_hash, test_user.password_hash);
        assert_eq!(user.secret, test_user.secret);
    }

    #[tokio::test]
    async fn test_find_by_id_user_not_exists() {
        let ctx = TestContext::new().await;
        let sut = UserRepositoryImpl::new(&ctx.pool);

        let res = sut.find_by_id(Uuid::new_v4()).await;

        assert!(res.is_none());
    }
}
