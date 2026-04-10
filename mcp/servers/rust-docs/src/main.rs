use rmcp::handler::server::router::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService,
    session::local::LocalSessionManager,
};
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod tools;
use tools::DocServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let ct = CancellationToken::new();

    let service = StreamableHttpService::new(
        || {
            let server = DocServer::new();
            let router = Router::new(server)
                .with_tools(DocServer::tool_router());
            Ok(router)
        },
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
    );

    let router = axum::Router::new()
        .route("/health", axum::routing::get(|| async { r#"{"status":"ok"}"# }))
        .fallback_service(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    tracing::info!("MCP rust-docs server listening on port 80");

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ct.cancel();
        })
        .await?;

    Ok(())
}
