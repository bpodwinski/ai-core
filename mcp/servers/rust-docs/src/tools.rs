use rmcp::ServerHandler;
use rmcp::model::ServerInfo;
use rmcp::schemars::JsonSchema;
use serde::Deserialize;

#[derive(Clone)]
struct DocEntry {
    framework: String,
    topic: String,
    content: String,
}

#[derive(Clone)]
pub struct DocServer {
    docs: Vec<DocEntry>,
}

// --- Tool input types ---

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchInput {
    /// Search query string
    pub query: String,
    /// Filter by framework: "leptos", "axum", or "rust" (optional)
    #[serde(default)]
    pub framework: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetDocInput {
    /// Framework name: "leptos", "axum", or "rust"
    pub framework: String,
    /// Topic name (use list_topics to see available topics)
    pub topic: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ListTopicsInput {
    /// Filter by framework: "leptos", "axum", or "rust" (optional, lists all if omitted)
    #[serde(default)]
    pub framework: Option<String>,
}

// --- Server implementation ---

impl DocServer {
    pub fn new() -> Self {
        let docs = vec![
            DocEntry {
                framework: "leptos".into(),
                topic: "getting-started".into(),
                content: include_str!("../docs/leptos/getting-started.md").into(),
            },
            DocEntry {
                framework: "leptos".into(),
                topic: "components".into(),
                content: include_str!("../docs/leptos/components.md").into(),
            },
            DocEntry {
                framework: "leptos".into(),
                topic: "signals".into(),
                content: include_str!("../docs/leptos/signals.md").into(),
            },
            DocEntry {
                framework: "axum".into(),
                topic: "getting-started".into(),
                content: include_str!("../docs/axum/getting-started.md").into(),
            },
            DocEntry {
                framework: "axum".into(),
                topic: "routing".into(),
                content: include_str!("../docs/axum/routing.md").into(),
            },
            DocEntry {
                framework: "axum".into(),
                topic: "extractors".into(),
                content: include_str!("../docs/axum/extractors.md").into(),
            },
            DocEntry {
                framework: "rust".into(),
                topic: "error-handling".into(),
                content: include_str!("../docs/rust/error-handling.md").into(),
            },
            DocEntry {
                framework: "rust".into(),
                topic: "async".into(),
                content: include_str!("../docs/rust/async.md").into(),
            },
        ];
        Self { docs }
    }
}

#[tool_router]
impl DocServer {
    #[tool(description = "Search documentation for Leptos, Axum, and Rust. Returns matching sections.")]
    async fn search_docs(&self, #[tool(aggr)] input: SearchInput) -> String {
        let query = input.query.to_lowercase();
        let results: Vec<_> = self
            .docs
            .iter()
            .filter(|doc| {
                let matches_query = doc.content.to_lowercase().contains(&query)
                    || doc.topic.to_lowercase().contains(&query);
                let matches_fw = input
                    .framework
                    .as_ref()
                    .map(|f| doc.framework == f.to_lowercase())
                    .unwrap_or(true);
                matches_query && matches_fw
            })
            .map(|doc| format!("## [{}/{}]\n\n{}", doc.framework, doc.topic, doc.content))
            .collect();

        if results.is_empty() {
            "No documentation found matching your query.".into()
        } else {
            results.join("\n\n---\n\n")
        }
    }

    #[tool(description = "Get a specific documentation page by framework and topic.")]
    async fn get_doc(&self, #[tool(aggr)] input: GetDocInput) -> String {
        let fw = input.framework.to_lowercase();
        let topic = input.topic.to_lowercase();
        self.docs
            .iter()
            .find(|doc| doc.framework == fw && doc.topic == topic)
            .map(|doc| doc.content.clone())
            .unwrap_or_else(|| {
                format!(
                    "Documentation not found: {}/{}. Use list_topics to see available topics.",
                    fw, topic
                )
            })
    }

    #[tool(description = "List available documentation topics, optionally filtered by framework.")]
    async fn list_topics(&self, #[tool(aggr)] input: ListTopicsInput) -> String {
        let topics: Vec<_> = self
            .docs
            .iter()
            .filter(|doc| {
                input
                    .framework
                    .as_ref()
                    .map(|f| doc.framework == f.to_lowercase())
                    .unwrap_or(true)
            })
            .map(|doc| format!("- {}/{}", doc.framework, doc.topic))
            .collect();

        if topics.is_empty() {
            "No documentation available.".into()
        } else {
            format!(
                "Available documentation ({} topics):\n{}",
                topics.len(),
                topics.join("\n")
            )
        }
    }
}

impl ServerHandler for DocServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Documentation server for Leptos, Axum, and Rust. \
                 Use search_docs to find information, get_doc to retrieve a specific page, \
                 and list_topics to see what's available."
                    .into(),
            ),
            ..Default::default()
        }
    }
}
