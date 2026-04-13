use crate::crate_proxy::CrateProxy;
use rmcp::handler::server::wrapper::{Json, Parameters};
use rmcp::model::{ServerCapabilities, ServerInfo};
use rmcp::schemars::JsonSchema;
use rmcp::{tool, tool_router, ServerHandler};
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
    crate_proxy: Option<Arc<CrateProxy>>,
}

// ── Static docs tool inputs ───────────────────────────────────────────────────

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

// ── Crate tool inputs ─────────────────────────────────────────────────────────

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct ListCachedCratesInput {}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CacheCrateInput {
    /// Name of the crate (e.g. "axum", "serde")
    pub crate_name: String,
    /// Source type: "cratesio", "github", or "local"
    pub source_type: String,
    /// Version string for cratesio (e.g. "0.8.4")
    #[serde(default)]
    pub version: Option<String>,
    /// GitHub URL for source_type "github" (e.g. "https://github.com/tokio-rs/axum")
    #[serde(default)]
    pub github_url: Option<String>,
    /// Branch name for source_type "github"
    #[serde(default)]
    pub branch: Option<String>,
    /// Tag name for source_type "github"
    #[serde(default)]
    pub tag: Option<String>,
    /// Filesystem path for source_type "local"
    #[serde(default)]
    pub path: Option<String>,
    /// For workspace crates: specific members to cache (e.g. ["crates/rmcp"])
    #[serde(default)]
    pub members: Option<Vec<String>>,
    /// Force re-download even if already cached
    #[serde(default)]
    pub update: Option<bool>,
    /// Specific features to enable instead of --all-features.
    /// Use for crates with mutually exclusive features (e.g. leptos-use: features: ["axum"]).
    /// When provided, uses --no-default-features --features=a,b,c.
    #[serde(default)]
    pub features: Option<Vec<String>>,
    /// Disable default features — LEGACY: prefer using features: [] instead.
    /// When true without features, defaults to features=[] (no-default-features).
    #[serde(default)]
    pub no_default_features: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Serialize, JsonSchema)]
pub struct CrateNameInput {
    /// Name of the cached crate
    pub crate_name: String,
    /// Version of the cached crate (e.g. "0.8.8")
    pub version: String,
    /// For workspace crates: member path (e.g. "crates/rmcp")
    #[serde(default)]
    pub member: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchCrateItemsInput {
    /// Name of the cached crate (e.g. "axum")
    pub crate_name: String,
    /// Version of the cached crate (e.g. "0.8.8")
    pub version: String,
    /// Pattern to search for in item names (e.g. "Router", "extract")
    pub pattern: String,
    /// Filter by item kind: "function", "struct", "trait", "enum", "type", "macro", "mod" (optional)
    #[serde(default)]
    pub kind_filter: Option<String>,
    /// Max results to return
    #[serde(default)]
    pub limit: Option<i64>,
    /// Pagination offset
    #[serde(default)]
    pub offset: Option<i64>,
    /// Filter by module path prefix (e.g. "axum::routing")
    #[serde(default)]
    pub path_filter: Option<String>,
    /// For workspace crates: member path
    #[serde(default)]
    pub member: Option<String>,
    /// Return only item IDs, names, and types (lighter response)
    #[serde(default)]
    pub preview: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetItemInput {
    /// Name of the cached crate
    pub crate_name: String,
    /// Version of the cached crate (e.g. "0.8.8")
    pub version: String,
    /// Numeric item ID (from search results)
    pub item_id: i32,
    /// For workspace crates: member path
    #[serde(default)]
    pub member: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GetItemSourceInput {
    /// Name of the cached crate
    pub crate_name: String,
    /// Version of the cached crate (e.g. "0.8.8")
    pub version: String,
    /// Numeric item ID (from search results)
    pub item_id: i32,
    /// Number of surrounding context lines (default: 3)
    #[serde(default)]
    pub context_lines: Option<i64>,
    /// For workspace crates: member path
    #[serde(default)]
    pub member: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListCrateItemsInput {
    /// Name of the cached crate
    pub crate_name: String,
    /// Version of the cached crate
    pub version: String,
    /// Filter by item kind: "function", "struct", "trait", "enum", "type", "macro", "mod"
    #[serde(default)]
    pub kind_filter: Option<String>,
    /// Max results (default 100)
    #[serde(default)]
    pub limit: Option<i64>,
    /// Pagination offset
    #[serde(default)]
    pub offset: Option<i64>,
    /// For workspace crates: member path
    #[serde(default)]
    pub member: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SearchItemsFuzzyInput {
    /// Name of the cached crate
    pub crate_name: String,
    /// Version of the cached crate
    pub version: String,
    /// Search query
    pub query: String,
    /// Filter by item kind
    #[serde(default)]
    pub kind_filter: Option<String>,
    /// Max results
    #[serde(default)]
    pub limit: Option<i64>,
    /// Edit distance for fuzzy matching (0-2, default 1)
    #[serde(default)]
    pub fuzzy_distance: Option<u8>,
    /// Enable fuzzy matching (default true)
    #[serde(default)]
    pub fuzzy_enabled: Option<bool>,
    /// For workspace crates: member path
    #[serde(default)]
    pub member: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct ListCrateVersionsInput {
    /// Name of the crate on crates.io
    pub crate_name: String,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct RemoveCrateInput {
    /// Name of the cached crate to remove
    pub crate_name: String,
    /// Version to remove
    pub version: String,
}

// ── Shared result type ────────────────────────────────────────────────────────

#[derive(Debug, Serialize, JsonSchema)]
pub struct DocResult {
    pub text: String,
}

// ── Filesystem loading ────────────────────────────────────────────────────────

pub fn load_docs_from_dir(base: &Path) -> Vec<DocEntry> {
    let mut docs = Vec::new();
    if !base.is_dir() {
        tracing::warn!("Docs directory not found: {}", base.display());
        return docs;
    }
    load_recursive(base, base, &mut docs);
    tracing::info!(
        "Loaded {} documentation pages from {}",
        docs.len(),
        base.display()
    );
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

// ── Server implementation ─────────────────────────────────────────────────────

impl DocServer {
    pub fn new(
        docs: Arc<Vec<DocEntry>>,
        categories: Vec<String>,
        crate_proxy: Option<Arc<CrateProxy>>,
    ) -> Self {
        Self {
            docs,
            categories,
            crate_proxy,
        }
    }

    fn full_path(doc: &DocEntry) -> String {
        if doc.category.is_empty() {
            doc.topic.clone()
        } else {
            format!("{}/{}", doc.category, doc.topic)
        }
    }

    fn proxy_unavailable() -> Json<DocResult> {
        Json(DocResult {
            text: "rust-docs-mcp subprocess is not available. Check server logs.".into(),
        })
    }
}

#[tool_router(vis = "pub")]
impl DocServer {
    // ── Static docs tools ─────────────────────────────────────────────────────

    #[tool(
        name = "search_docs",
        description = "Search documentation across all loaded docs. Returns matching sections. Use the optional 'category' parameter to filter by doc source (e.g. 'leptos', 'rust', 'daisyui')."
    )]
    fn search_docs(&self, Parameters(input): Parameters<SearchInput>) -> Json<DocResult> {
        if input.query.len() > 500 {
            return Json(DocResult {
                text: "Query too long (max 500 characters).".into(),
            });
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
            format!(
                "{} result(s):\n\n{}",
                results.len(),
                results.join("\n\n---\n\n")
            )
        };
        Json(DocResult { text })
    }

    #[tool(
        name = "get_doc",
        description = "Get a specific documentation page by its full path (category/topic). Use list_topics to discover available paths."
    )]
    fn get_doc(&self, Parameters(input): Parameters<GetDocInput>) -> Json<DocResult> {
        if input.path.contains("..") || input.path.starts_with('/') {
            return Json(DocResult {
                text: "Invalid path.".into(),
            });
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

    #[tool(
        name = "list_topics",
        description = "List available documentation topics, optionally filtered by category prefix."
    )]
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

    // ── Crate tools (delegated to rust-docs-mcp subprocess) ──────────────────
    // Use block_in_place to call async proxy methods from sync tool handlers
    // (rmcp 1.3 tool_router does not support async fn).

    #[tool(
        name = "cache_crate",
        description = "Download and cache a Rust crate for local analysis. Use source_type 'cratesio' with a version, 'github' with github_url and branch/tag, or 'local' with a path. Use 'features' for crates with mutually exclusive features (e.g. features: ['axum'] for leptos-use)."
    )]
    fn cache_crate(&self, Parameters(input): Parameters<CacheCrateInput>) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };

        // Resolve features: explicit features take priority, no_default_features=true
        // is treated as features=[] for backward compatibility.
        let features = if input.features.is_some() {
            input.features.clone()
        } else if input.no_default_features == Some(true) {
            Some(vec![])
        } else {
            None
        };

        let args = serde_json::json!({
            "crate_name":  input.crate_name,
            "source_type": input.source_type,
            "version":     input.version,
            "github_url":  input.github_url,
            "branch":      input.branch,
            "tag":         input.tag,
            "path":        input.path,
            "members":     input.members,
            "update":      input.update,
            "features":    features,
        });
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool("cache_crate", args))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "list_cached_crates",
        description = "List all locally cached Rust crates with their versions and sizes."
    )]
    fn list_cached_crates(&self, _: Parameters<ListCachedCratesInput>) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(proxy.call_tool("list_cached_crates", serde_json::json!({})))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "search_crate_items",
        description = "Search items (functions, structs, traits, etc.) in a cached Rust crate. Returns matching API items with documentation."
    )]
    fn search_crate_items(
        &self,
        Parameters(input): Parameters<SearchCrateItemsInput>,
    ) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let tool = if input.preview.unwrap_or(false) {
            "search_items_preview"
        } else {
            "search_items"
        };
        let mut args = serde_json::json!({
            "crate_name": input.crate_name,
            "version": input.version,
            "pattern": input.pattern,
        });
        if let Some(kind_filter) = input.kind_filter {
            args["kind_filter"] = serde_json::Value::String(kind_filter);
        }
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(tool, args))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "get_item_details",
        description = "Get detailed information about a specific item in a cached crate: signature, fields, methods, and documentation."
    )]
    fn get_item_details(&self, Parameters(input): Parameters<GetItemInput>) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "get_item_details",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "get_item_source",
        description = "View the source code of a specific item in a cached crate, with configurable surrounding context lines."
    )]
    fn get_item_source(
        &self,
        Parameters(input): Parameters<GetItemSourceInput>,
    ) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "get_item_source",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "get_crate_dependencies",
        description = "Analyze the direct and transitive dependencies of a cached Rust crate."
    )]
    fn get_crate_dependencies(
        &self,
        Parameters(input): Parameters<CrateNameInput>,
    ) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "get_dependencies",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "list_crate_items",
        description = "List all items (functions, structs, traits, etc.) in a cached Rust crate. Supports pagination and kind filtering."
    )]
    fn list_crate_items(
        &self,
        Parameters(input): Parameters<ListCrateItemsInput>,
    ) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "list_crate_items",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "get_item_docs",
        description = "Get the documentation string for a specific item in a cached crate (lighter than get_item_details)."
    )]
    fn get_item_docs(&self, Parameters(input): Parameters<GetItemInput>) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "get_item_docs",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "search_crate_items_fuzzy",
        description = "Fuzzy search for items in a cached Rust crate. Tolerates typos (configurable edit distance)."
    )]
    fn search_crate_items_fuzzy(
        &self,
        Parameters(input): Parameters<SearchItemsFuzzyInput>,
    ) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "search_items_fuzzy",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "list_crate_versions",
        description = "List available versions of a crate on crates.io."
    )]
    fn list_crate_versions(
        &self,
        Parameters(input): Parameters<ListCrateVersionsInput>,
    ) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "list_crate_versions",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }

    #[tool(
        name = "remove_crate",
        description = "Remove a cached Rust crate from local storage to free up space."
    )]
    fn remove_crate(&self, Parameters(input): Parameters<RemoveCrateInput>) -> Json<DocResult> {
        let Some(proxy) = self.crate_proxy.clone() else {
            return Self::proxy_unavailable();
        };
        let text = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(proxy.call_tool(
                "remove_crate",
                serde_json::to_value(input).unwrap_or_default(),
            ))
        });
        Json(DocResult { text })
    }
}

impl ServerHandler for DocServer {
    fn get_info(&self) -> ServerInfo {
        let proxy_status = if self.crate_proxy.is_some() {
            "Crate tools (rust-docs-mcp): cache_crate, list_cached_crates, list_crate_versions, \
             search_crate_items, search_crate_items_fuzzy, list_crate_items, \
             get_item_details, get_item_docs, get_item_source, get_crate_dependencies, remove_crate."
        } else {
            "Crate tools unavailable (rust-docs-mcp not found)."
        };
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder().enable_tools().build();
        info.instructions = Some(format!(
            "Documentation MCP server serving {} pages across {} categories: {}. \
             Use search_docs to find information (filter by category with the 'category' parameter), \
             get_doc to retrieve a specific page by path, \
             and list_topics to see available content. \
             {}",
            self.docs.len(),
            self.categories.len(),
            self.categories.join(", "),
            proxy_status,
        ));
        info
    }
}
