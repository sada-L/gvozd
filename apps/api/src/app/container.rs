use actix_web::{dev::Server, web};
use application::services::auth::AuthService;
use infrastructure::{
    db,
    repository::{token::TokenServiceImpl, user::UserRepositoryImpl},
};
use log::info;
use presentation::api::auth::AuthController;
use std::sync::Arc;

use crate::app;

pub struct AppContainer {
    pub server: Server,
}

pub struct Dependency {
    pub auth_controller: web::Data<AuthController>,
}

impl AppContainer {
    pub async fn new() -> Self {
        dotenv::from_filename(".env.local").ok();
        env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
        let config = app::config::AppConfig::from_env().expect("Error to build config");
        info!("Configuration loaded successfully");

        info!("Connecting to database...");
        let pool = db::init_db_pool(&config.database.url, config.database.max_connections)
            .await
            .expect("Error connect to database");
        info!("Database connected successfully");

        info!("Start migration...");
        db::run_migration(&pool, "./migrations").await;
        info!("Migrations accept successfully");

        let http_adds = format! {"{}:{}", config.server.http_host, config.server.http_port};

        let user_repository = Arc::new(UserRepositoryImpl::new(&pool));
        let token_service = Arc::new(TokenServiceImpl::new(config.auth.secret, config.auth.exp));

        let auth_service = Arc::new(AuthService::new(
            user_repository.clone(),
            token_service.clone(),
        ));

        let auth_controller = web::Data::new(AuthController::new(auth_service.clone()));

        let deps = Dependency { auth_controller };

        let server =
            app::server::build_http_server(&deps, &http_adds).expect("Failed to build server");
        info!(
            "Server start at {}:{}",
            config.server.http_host, config.server.http_port
        );

        Self { server }
    }
}
