use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

use crate::config::AppConfig;

pub async fn router() -> Result<Router, anyhow::Error> {
    // config load
    #[allow(unused)]
    let cfg =
        AppConfig::new().expect("Configuration initialization failed, check pg .env settings.");

    // app init
    Ok(Router::new()
        .route("/", get(index))
        .route("/health-check", get(health_check)))
}

async fn index() -> impl IntoResponse {
    "Temporary Index Result"
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "duuuuu~")
}
