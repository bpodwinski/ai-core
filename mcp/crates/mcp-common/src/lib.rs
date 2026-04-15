//! Shared data types for the MCP workspace.
//!
//! Provides serializable structures that represent `servers-manifest.json`,
//! the single source of truth consumed by both the build toolchain (`mcp-build`,
//! `mcp-configgen`) and the runtime server (`mcp-server`).

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// Top-level manifest describing a self-hosted MCP deployment.
///
/// Loaded from `servers-manifest.json` at build time by `mcp-configgen` and
/// `mcp-build`, and at startup by `mcp-server`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    /// Public base URL of the deployment (e.g. `"https://mcp.example.com"`).
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    /// Path suffix appended to `base_url` for the MCP endpoint (e.g. `"/mcp"`).
    #[serde(rename = "mcpEndpoint")]
    pub mcp_endpoint: String,
    /// Human-readable server name shown in generated client configuration files.
    #[serde(rename = "mcpName")]
    pub mcp_name: String,
    /// Documentation sources fetched by `mcp-build` and served as MCP categories.
    #[serde(rename = "docSources", default)]
    pub doc_sources: Vec<DocSourceEntry>,
    /// External HTTP MCP servers referenced in generated client configs.
    #[serde(default)]
    pub external: Vec<ExternalServer>,
    /// Stdio MCP servers (e.g. `npx` packages) referenced in generated client configs.
    #[serde(default)]
    pub stdio: Vec<StdioServer>,
}

/// A single documentation source entry declared in the manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSourceEntry {
    /// Category identifier — used as the subdirectory name under `/docs/` and
    /// as the filter key in MCP tool calls.
    pub name: String,
    /// Short human-readable description of the documentation category.
    #[serde(default)]
    pub description: String,
    /// Origin and fetch strategy for this documentation source.
    pub source: Option<DocSource>,
}

/// Origin and fetch strategy for a documentation source.
///
/// Determines how `mcp-build fetch-docs` retrieves and prepares the Markdown
/// files that the server loads at startup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DocSource {
    /// Clone or update a Git repository and copy files from a subdirectory.
    Git {
        /// HTTPS URL of the Git repository to clone.
        url: String,
        /// Path inside the repository that contains the Markdown files.
        #[serde(rename = "docsPath")]
        docs_path: String,
        /// Post-processing transforms applied in order after cloning.
        #[serde(default)]
        transforms: Vec<Transform>,
    },
    /// Download a single remote file (e.g. `llms.txt`) over HTTP/HTTPS.
    Url {
        /// Full HTTP/HTTPS URL of the file to download.
        url: String,
        /// Post-processing transforms applied in order after downloading.
        #[serde(default)]
        transforms: Vec<Transform>,
    },
    /// Use pre-existing Markdown files from a local path inside the build context.
    Local {
        /// Path relative to `crates/mcp-server/local-docs/`.
        path: String,
        /// Post-processing transforms applied in order.
        #[serde(default)]
        transforms: Vec<Transform>,
    },
}

/// Post-processing transform applied to raw documentation files by `mcp-build`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Transform {
    /// Strip MDX syntax (imports, exports, JSX expressions) and convert to plain Markdown.
    StripMdx,
    /// Generate a Tailwind CSS class catalog from the source files.
    GenerateCatalog,
    /// Split a single large file into one Markdown file per top-level heading.
    Split,
}

/// An external HTTP MCP server to reference in generated client configuration files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServer {
    /// Identifier used as the server key in `.mcp.json`.
    pub name: String,
    /// Full HTTPS URL of the server's MCP endpoint.
    pub url: String,
    /// Human-readable description shown in generated configs.
    #[serde(default)]
    pub description: String,
    /// Name of the environment variable that holds the Bearer token, if the
    /// server requires authentication.
    #[serde(
        rename = "bearer_token_env_var",
        skip_serializing_if = "Option::is_none"
    )]
    pub bearer_token_env_var: Option<String>,
}

/// A stdio-based MCP server launched as a local child process (e.g. via `npx`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioServer {
    /// Identifier used as the server key in `.mcp.json`.
    pub name: String,
    /// Executable to invoke (e.g. `"npx"`, `"node"`).
    pub command: String,
    /// Arguments forwarded to `command`.
    #[serde(default)]
    pub args: Vec<String>,
    /// Human-readable description shown in generated configs.
    #[serde(default)]
    pub description: String,
    /// Optional environment variables injected into the child process.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,
}

impl Manifest {
    /// Load and deserialize a manifest from the JSON file at `path`.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or if the JSON is malformed.
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("reading manifest at {}", path.display()))?;
        let manifest: Manifest = serde_json::from_str(&raw)
            .with_context(|| format!("parsing manifest at {}", path.display()))?;
        Ok(manifest)
    }

    /// Validate manifest consistency.
    ///
    /// Checks that:
    /// - `base_url` is not empty.
    /// - Every `docSources` entry has a unique `name` (duplicates would silently
    ///   overwrite each other's output directory at build time).
    ///
    /// # Errors
    ///
    /// Returns an error describing the first violation found.
    pub fn validate(&self) -> Result<()> {
        if self.base_url.is_empty() {
            anyhow::bail!("manifest: `baseUrl` must not be empty");
        }
        let mut seen = std::collections::HashSet::new();
        for entry in &self.doc_sources {
            if !seen.insert(&entry.name) {
                anyhow::bail!(
                    "manifest: duplicate docSource name {:?} — each name must be unique",
                    entry.name
                );
            }
        }
        Ok(())
    }
}

impl DocSource {
    /// Return the list of transforms configured for this source variant.
    pub fn transforms(&self) -> &[Transform] {
        match self {
            DocSource::Git { transforms, .. }
            | DocSource::Url { transforms, .. }
            | DocSource::Local { transforms, .. } => transforms,
        }
    }
}
