use platform_core::{PlatformError, PlatformResult};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_env: String,
    pub app_name: String,
    pub api_host: String,
    pub api_port: u16,
    pub database_url: String,
    pub redis_url: String,
}

impl AppConfig {
    pub fn from_env() -> PlatformResult<Self> {
        Ok(Self {
            app_env: read_env("APP_ENV", "local"),
            app_name: read_env("APP_NAME", "universal-platform"),
            api_host: read_env("API_HOST", "0.0.0.0"),
            api_port: read_env("API_PORT", "8080")
                .parse::<u16>()
                .map_err(|error| PlatformError::Configuration(format!("API_PORT must be a valid u16: {error}")))?,
            database_url: read_env(
                "DATABASE_URL",
                "postgres://platform:platform@localhost:5432/platform_db",
            ),
            redis_url: read_env("REDIS_URL", "redis://localhost:6379"),
        })
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}

fn read_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_owned())
}
