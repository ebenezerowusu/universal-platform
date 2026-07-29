use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use platform_config::AppConfig;
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct ApiState {
    config: Arc<AppConfig>,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T>
where
    T: Serialize,
{
    success: bool,
    data: T,
    meta: ResponseMeta,
}

#[derive(Debug, Serialize)]
struct ResponseMeta {
    service: &'static str,
    trace_id: &'static str,
}

#[derive(Debug, Serialize)]
struct HealthData {
    status: &'static str,
}

#[derive(Debug, Serialize)]
struct ReadyData {
    status: &'static str,
    database: &'static str,
    redis: &'static str,
    storage: &'static str,
}

#[tokio::main]
async fn main() {
    init_tracing();

    let config = match AppConfig::from_env() {
        Ok(config) => config,
        Err(error) => {
            error!(%error, "failed to load application configuration");
            std::process::exit(1);
        }
    };

    let bind_address = config.bind_address();
    let state = ApiState {
        config: Arc::new(config),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .with_state(state);

    let socket_address: SocketAddr = match bind_address.parse() {
        Ok(address) => address,
        Err(error) => {
            error!(%error, %bind_address, "invalid API bind address");
            std::process::exit(1);
        }
    };

    let listener = match tokio::net::TcpListener::bind(socket_address).await {
        Ok(listener) => listener,
        Err(error) => {
            error!(%error, %socket_address, "failed to bind API listener");
            std::process::exit(1);
        }
    };

    info!(%socket_address, "platform API started");

    if let Err(error) = axum::serve(listener, app).await {
        error!(%error, "platform API stopped unexpectedly");
        std::process::exit(1);
    }
}

async fn health(State(_state): State<ApiState>) -> impl IntoResponse {
    Json(ApiResponse {
        success: true,
        data: HealthData { status: "ok" },
        meta: ResponseMeta {
            service: "platform-api",
            trace_id: "not-wired-yet",
        },
    })
}

async fn ready(State(state): State<ApiState>) -> impl IntoResponse {
    let data = ReadyData {
        status: "degraded-placeholder",
        database: if state.config.database_url.is_empty() {
            "not-configured"
        } else {
            "configured-not-checked"
        },
        redis: if state.config.redis_url.is_empty() {
            "not-configured"
        } else {
            "configured-not-checked"
        },
        storage: "local-placeholder-not-checked",
    };

    (StatusCode::OK, Json(ApiResponse {
        success: true,
        data,
        meta: ResponseMeta {
            service: "platform-api",
            trace_id: "not-wired-yet",
        },
    }))
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
