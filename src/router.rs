use axum::Router;

pub async fn router() -> Result<Router, anyhow::Error> {
    // app init
    Ok(Router::new())
}
