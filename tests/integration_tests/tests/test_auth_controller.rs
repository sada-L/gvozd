#[cfg(test)]
mod test {
    use actix_web::{App, test, web};
    use application::services::auth::AuthService;
    use infrastructure::repository::{token::TokenServiceImpl, user::UserRepositoryImpl};
    use integration_tests::TestContext;
    use presentation::api::auth::AuthController;
    use serde_json::json;
    use std::sync::Arc;

    #[actix_web::test]
    async fn test_signup_success() {
        let ctx = TestContext::new().await;

        let user_repo = Arc::new(UserRepositoryImpl::new(&ctx.pool));
        let token_serv = Arc::new(TokenServiceImpl::new("secret".to_string(), 10));
        let auth_service = Arc::new(AuthService::new(user_repo.clone(), token_serv.clone()));
        let sut = web::Data::new(AuthController::new(auth_service.clone()));

        let app = test::init_service(
            App::new()
                .app_data(sut)
                .route("/auth/signup", web::post().to(AuthController::signup)),
        )
        .await;

        let payload = json!({
            "email": "test@example.com",
            "password": "testpassword",
            "username": "Test User",
        });

        let req = test::TestRequest::post()
            .uri("/auth/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = test::read_body_json(resp).await;

        assert!(body.get("access_token").is_some());
        assert!(body.get("refresh_token").is_some());
        assert!(body.get("unexpected_data").is_none());
    }

    #[actix_web::test]
    async fn test_signup_failded() {
        let ctx = TestContext::new().await;

        let user_repo = Arc::new(UserRepositoryImpl::new(&ctx.pool));
        let token_serv = Arc::new(TokenServiceImpl::new("secret".to_string(), 10));
        let auth_service = Arc::new(AuthService::new(user_repo.clone(), token_serv.clone()));
        let sut = web::Data::new(AuthController::new(auth_service.clone()));

        let app = test::init_service(
            App::new()
                .app_data(sut)
                .route("/auth/signup", web::post().to(AuthController::signup)),
        )
        .await;

        let payload = json!({
            "email": "test@example.com",
            "password": "testpassword",
            "username": "Test User",
        });

        let req = test::TestRequest::post()
            .uri("/auth/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 200);

        let req = test::TestRequest::post()
            .uri("/auth/signup")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), 400);
    }
}
