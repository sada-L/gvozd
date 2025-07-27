#[cfg(test)]
mod test {
    use application::{dto::user::UserDTO, services::auth::AuthService};
    use async_trait::async_trait;
    use domain::{
        models::token::{Token, TokenClaims, TokenError},
        models::user::{User, UserError},
        repository::{token::TokenService, user::UserRepository},
    };
    use mockall::mock;
    use std::sync::Arc;
    use uuid::Uuid;

    mock! {
        UserRepo {}

        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create(&self, user: &User) -> Result<(), UserError>;
            async fn update(&self, user: &User) -> Result<(), UserError>;
            async fn delete(&self, id: Uuid) -> Result<(), UserError>;
            async fn find_by_id(&self, id: uuid::Uuid) -> Option<User>;
            async fn find_by_email(&self, email: &str) -> Option<User>;
        }
    }

    mock! {
        TokenServ{}

        impl TokenService for TokenServ {
            fn generate_token(&self, user: &User) -> Result<Token, TokenError>;
            fn verify_token(&self, token: &str) -> Result<TokenClaims, TokenError>;
        }
    }

    #[tokio::test]
    async fn signup_should_call_repository_with_correct_user() {
        let mut repo = MockUserRepo::new();
        let mut jwt = MockTokenServ::new();
        let expected_user: User = User::default();
        let user_dto: UserDTO = expected_user.clone().into();

        repo.expect_create()
            .withf(move |user| user.email.value() == expected_user.email.value())
            .times(1)
            .returning(|_| Ok(()));

        jwt.expect_generate_token()
            .returning(|_| Ok(Token::default()));

        let service = AuthService::new(Arc::new(repo), Arc::new(jwt));

        let result = service.signup(user_dto).await;

        assert!(result.is_ok());
    }
}
