//! MCP documentation server binary.
//!
//! Loads all Markdown documentation from `DOCS_PATH` at startup, exposes them
//! through the `rmcp` Streamable HTTP transport, and handles OAuth 2.1 PKCE
//! authorization in the same process.

use rmcp::handler::server::router::Router;
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod crate_proxy;
mod oauth;
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

    // Try to spawn rust-docs-mcp as a subprocess — degrade gracefully if unavailable
    let crate_proxy = match crate_proxy::CrateProxy::spawn().await {
        Ok(p) => {
            tracing::info!("rust-docs-mcp subprocess ready");
            Some(p)
        }
        Err(e) => {
            tracing::warn!("rust-docs-mcp unavailable, crate tools disabled: {e}");
            None
        }
    };

    let oauth_state = oauth::OAuthState::from_env()?;
    tracing::info!("OAuth endpoints mounted");

    let ct = CancellationToken::new();

    // Health check captures doc stats
    let doc_count = docs.len();
    let health_cats = Arc::new(categories.clone());
    let proxy_available = crate_proxy.is_some();

    let service = StreamableHttpService::new(
        move || {
            let server = DocServer::new(docs.clone(), categories.clone(), crate_proxy.clone());
            let router = Router::new(server).with_tools(DocServer::tool_router());
            Ok(router)
        },
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default().with_cancellation_token(ct.child_token()),
    );

    let router = axum::Router::new()
        .route(
            "/health",
            axum::routing::get(move || {
                let cats = health_cats.clone();
                async move {
                    axum::Json(serde_json::json!({
                        "status": "ok",
                        "docs_loaded": doc_count,
                        "categories": *cats,
                        "crate_tools": proxy_available,
                    }))
                }
            }),
        )
        .merge(oauth::router(oauth_state))
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
