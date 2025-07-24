use infrastructure::db;
use sqlx::PgPool;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};

pub struct TestContext {
    _container: ContainerAsync<Postgres>,
    pub pool: PgPool,
}

impl TestContext {
    pub async fn new() -> Self {
        let postgres = Postgres::default()
            .with_db_name("test_db")
            .with_user("test_user")
            .with_password("test_password");

        let container = postgres.start().await.unwrap();

        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let database_url = format!("postgresql://test_user:test_password@localhost:{port}/test_db");

        let pool = db::init_db_pool(&database_url, 10).await.unwrap();
        db::run_migration(&pool, "../../migrations/").await;

        Self {
            _container: container,
            pool,
        }
    }
}
