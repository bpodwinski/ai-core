use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "mcpEndpoint")]
    pub mcp_endpoint: String,
    #[serde(rename = "mcpName")]
    pub mcp_name: String,
    #[serde(rename = "docSources", default)]
    pub doc_sources: Vec<DocSourceEntry>,
    #[serde(default)]
    pub external: Vec<ExternalServer>,
    #[serde(default)]
    pub stdio: Vec<StdioServer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSourceEntry {
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub source: Option<DocSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum DocSource {
    Git {
        url: String,
        #[serde(rename = "docsPath")]
        docs_path: String,
        #[serde(default)]
        transforms: Vec<Transform>,
    },
    Url {
        url: String,
        #[serde(default)]
        transforms: Vec<Transform>,
    },
    Local {
        path: String,
        #[serde(default)]
        transforms: Vec<Transform>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum Transform {
    StripMdx,
    GenerateCatalog,
    Split,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServer {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub description: String,
    #[serde(
        rename = "bearer_token_env_var",
        skip_serializing_if = "Option::is_none"
    )]
    pub bearer_token_env_var: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StdioServer {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<BTreeMap<String, String>>,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        let raw = std::fs::read_to_string(path)
            .with_context(|| format!("reading manifest at {}", path.display()))?;
        let manifest: Manifest = serde_json::from_str(&raw)
            .with_context(|| format!("parsing manifest at {}", path.display()))?;
        Ok(manifest)
    }
}

impl DocSource {
    pub fn transforms(&self) -> &[Transform] {
        match self {
            DocSource::Git { transforms, .. }
            | DocSource::Url { transforms, .. }
            | DocSource::Local { transforms, .. } => transforms,
        }
    }
}
