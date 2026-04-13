use rmcp::handler::server::router::Router;
use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService,
    session::local::LocalSessionManager,
};
use std::sync::Arc;
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

    // Load all docs once at startup and share across sessions via Arc
    let docs_path = std::env::var("DOCS_PATH").unwrap_or_else(|_| "/docs".into());
    let docs = Arc::new(tools::load_docs_from_dir(std::path::Path::new(&docs_path)));
    let categories = tools::extract_categories(&docs);
    tracing::info!(
        "Loaded {} docs across {} categories: {}",
        docs.len(),
        categories.len(),
        categories.join(", ")
    );

    let ct = CancellationToken::new();

    // Health check captures doc stats
    let doc_count = docs.len();
    let health_cats = Arc::new(categories.clone());

    let service = StreamableHttpService::new(
        move || {
            let server = DocServer::with_docs(docs.clone(), categories.clone());
            let router = Router::new(server)
                .with_tools(DocServer::tool_router());
            Ok(router)
        },
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
    );

    let router = axum::Router::new()
        .route("/health", axum::routing::get(move || {
            let cats = health_cats.clone();
            async move {
                axum::Json(serde_json::json!({
                    "status": "ok",
                    "docs_loaded": doc_count,
                    "categories": *cats
                }))
            }
        }))
        .fallback_service(service);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:80").await?;
    tracing::info!("MCP docs server listening on port 80");

    axum::serve(listener, router)
        .with_graceful_shutdown(async move {
            tokio::signal::ctrl_c().await.unwrap();
            ct.cancel();
        })
        .await?;

    Ok(())
}
