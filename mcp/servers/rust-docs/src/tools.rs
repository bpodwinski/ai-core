use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::schemars::JsonSchema;
use rmcp::{ServerHandler, tool, tool_router};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub(crate) struct DocEntry {
    category: String,
    topic: String,
    content: String,
}

#[derive(Clone)]
pub struct DocServer {
    docs: Arc<Vec<DocEntry>>,
    categories: Vec<String>,
}

// --- Tool input/output types ---

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct SearchInput {
    /// Search query string
    pub query: String,
    /// Filter by category prefix, e.g. "leptos-use", "axum", "rust" (optional)
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct GetDocInput {
    /// Full path: "category/topic", e.g. "leptos-use/animation/use_interval"
    pub path: String,
}

#[derive(Debug, Default, Deserialize, JsonSchema)]
pub struct ListTopicsInput {
    /// Filter by category prefix, e.g. "leptos-use", "leptos-use/animation", "axum" (optional)
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct DocResult {
    pub text: String,
}

// --- Load docs from filesystem ---

pub fn load_docs_from_dir(base: &Path) -> Vec<DocEntry> {
    let mut docs = Vec::new();
    if !base.is_dir() {
        tracing::warn!("Docs directory not found: {}", base.display());
        return docs;
    }
    load_recursive(base, base, &mut docs);
    tracing::info!("Loaded {} documentation pages from {}", docs.len(), base.display());
    docs
}

pub fn extract_categories(docs: &[DocEntry]) -> Vec<String> {
    let mut cats = BTreeSet::new();
    for doc in docs {
        let top = doc.category.split('/').next().unwrap_or(&doc.category);
        if !top.is_empty() {
            cats.insert(top.to_string());
        }
    }
    cats.into_iter().collect()
}

fn load_recursive(base: &Path, current: &Path, docs: &mut Vec<DocEntry>) {
    let entries = match std::fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            load_recursive(base, &path, docs);
        } else if path.extension().is_some_and(|e| e == "md") {
            let rel = path.strip_prefix(base).unwrap_or(&path);
            let category = rel
                .parent()
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_default();
            let topic = path
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            if let Ok(content) = std::fs::read_to_string(&path) {
                docs.push(DocEntry {
                    category,
                    topic,
                    content,
                });
            }
        }
    }
}

// --- Server implementation ---

impl DocServer {
    pub fn with_docs(docs: Arc<Vec<DocEntry>>, categories: Vec<String>) -> Self {
        Self { docs, categories }
    }

    fn full_path(doc: &DocEntry) -> String {
        if doc.category.is_empty() {
            doc.topic.clone()
        } else {
            format!("{}/{}", doc.category, doc.topic)
        }
    }
}

#[tool_router(vis = "pub")]
impl DocServer {
    #[tool(name = "search_docs", description = "Search documentation across all loaded docs. Returns matching sections. Use the optional 'category' parameter to filter by doc source (e.g. 'leptos', 'rust', 'daisyui').")]
    fn search_docs(&self, Parameters(input): Parameters<SearchInput>) -> Json<DocResult> {
        if input.query.len() > 500 {
            return Json(DocResult { text: "Query too long (max 500 characters).".into() });
        }
        let query = input.query.to_lowercase();
        let results: Vec<_> = self
            .docs
            .iter()
            .filter(|doc| {
                let matches_query = doc.content.to_lowercase().contains(&query)
                    || doc.topic.to_lowercase().contains(&query)
                    || doc.category.to_lowercase().contains(&query);
                let matches_cat = input
                    .category
                    .as_ref()
                    .map(|c| doc.category.to_lowercase().starts_with(&c.to_lowercase()))
                    .unwrap_or(true);
                matches_query && matches_cat
            })
            .take(20)
            .map(|doc| format!("## [{}]\n\n{}", Self::full_path(doc), doc.content))
            .collect();

        let text = if results.is_empty() {
            "No documentation found matching your query.".into()
        } else {
            format!("{} result(s):\n\n{}", results.len(), results.join("\n\n---\n\n"))
        };
        Json(DocResult { text })
    }

    #[tool(name = "get_doc", description = "Get a specific documentation page by its full path (category/topic). Use list_topics to discover available paths.")]
    fn get_doc(&self, Parameters(input): Parameters<GetDocInput>) -> Json<DocResult> {
        if input.path.contains("..") || input.path.starts_with('/') {
            return Json(DocResult { text: "Invalid path.".into() });
        }
        let path = input.path.to_lowercase();
        let text = self
            .docs
            .iter()
            .find(|doc| Self::full_path(doc).to_lowercase() == path)
            .map(|doc| doc.content.clone())
            .unwrap_or_else(|| {
                format!(
                    "Documentation not found: {}. Use list_topics to see available paths.",
                    input.path
                )
            });
        Json(DocResult { text })
    }

    #[tool(name = "list_topics", description = "List available documentation topics, optionally filtered by category prefix.")]
    fn list_topics(&self, Parameters(input): Parameters<ListTopicsInput>) -> Json<DocResult> {
        let topics: Vec<_> = self
            .docs
            .iter()
            .filter(|doc| {
                input
                    .category
                    .as_ref()
                    .map(|c| {
                        let c = c.to_lowercase();
                        doc.category.to_lowercase().starts_with(&c)
                            || Self::full_path(doc).to_lowercase().starts_with(&c)
                    })
                    .unwrap_or(true)
            })
            .map(|doc| format!("- {}", Self::full_path(doc)))
            .collect();

        let text = if topics.is_empty() {
            "No documentation available.".into()
        } else {
            format!(
                "Available documentation ({} topics):\n{}",
                topics.len(),
                topics.join("\n")
            )
        };
        Json(DocResult { text })
    }
}

impl ServerHandler for DocServer {
    fn get_info(&self) -> ServerInfo {
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.instructions = Some(format!(
            "Documentation MCP server serving {} pages across {} categories: {}. \
             Use search_docs to find information (filter by category with the 'category' parameter), \
             get_doc to retrieve a specific page by path, \
             and list_topics to see available content.",
            self.docs.len(),
            self.categories.len(),
            self.categories.join(", ")
        ));
        info
    }
}
