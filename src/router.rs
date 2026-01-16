use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

pub async fn router() -> Result<Router, anyhow::Error> {
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
