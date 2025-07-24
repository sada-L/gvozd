use sqlx::{PgPool, migrate::Migrator, postgres::PgPoolOptions};
use std::{path::Path, time::Duration};

pub async fn init_db_pool(url: &str, max_conn: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_conn)
        .acquire_timeout(Duration::from_secs(10))
        .connect(url)
        .await
}

pub async fn init_db_pool_with_retry(url: &str, max_conn: u32) -> PgPool {
    loop {
        match PgPoolOptions::new()
            .max_connections(max_conn)
            .acquire_timeout(Duration::from_secs(10))
            .connect(url)
            .await
        {
            Ok(pool) => break pool,
            Err(e) => {
                println!("Waiting for DB... {e}");
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
    }
}

pub async fn run_migration(pool: &PgPool, path: &str) {
    log::info!("Migrating and configuring database...");
    Migrator::new(Path::new(path))
        .await
        .expect("Failed to get migrations config")
        .run(pool)
        .await
        .expect("Failed to apply migrations")
}
