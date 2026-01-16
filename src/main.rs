use emne::router;
use emne::utils::gracefully_shutdown;
use emne::utils::tracing_setup;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_setup::init_tracing_subscriber();

    // server build
    let http_addr: std::net::SocketAddr = "0.0.0.0:7878".parse()?;

    tracing::info!("http listening on http://{http_addr}");

    let token = gracefully_shutdown::shutdown_token();
    let app = router::router().await?;
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async move { token.cancelled().await })
        .await?;

    Ok(())
}
