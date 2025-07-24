use std::env;

use crate::app::errors::{StartupError, StartupResult};

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
}

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

pub struct ServerConfig {
    pub http_host: String,
    pub http_port: u16,
}

pub struct AuthConfig {
    pub secret: String,
    pub exp: i64,
}

impl AppConfig {
    pub fn from_env() -> StartupResult<Self> {
        Ok(Self {
            database: DatabaseConfig::from_env()?,
            server: ServerConfig::from_env()?,
            auth: AuthConfig::from_env()?,
        })
    }
}

impl DatabaseConfig {
    fn from_env() -> StartupResult<Self> {
        Ok(Self {
            url: env::var("DATABASE_URL")
                .map_err(|_| StartupError::EnvVarMissing("DATABASE_URL".to_string()))?,
            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .map_err(|_| {
                    StartupError::Configuration("Invalid DATABASE_MAX_CONNECTIONS".to_string())
                })?,
        })
    }
}

impl ServerConfig {
    fn from_env() -> StartupResult<Self> {
        Ok(Self {
            http_host: env::var("HTTP_HOST").unwrap_or("127.0.0.1".to_string()),
            http_port: env::var("HTTP_PORT")
                .unwrap_or("8080".to_string())
                .parse()
                .map_err(|_| StartupError::Configuration("Invalid HTTP_PORT".to_string()))?,
        })
    }
}

impl AuthConfig {
    fn from_env() -> StartupResult<Self> {
        Ok(Self {
            secret: env::var("secret_key").unwrap_or("secret_key".to_string()),
            exp: env::var("MAX_AGE")
                .unwrap_or("604800".to_string())
                .parse()
                .map_err(|_| StartupError::Configuration("Invalid MAX_AGE".to_string()))?,
        })
    }
}
