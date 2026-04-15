// Example 02 — BEFORE: error enum and fallible functions, nothing documented.

use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("not found: {0}")]
    NotFound(String),

    #[error("permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub fn load(key: &str) -> Result<Vec<u8>, StoreError> {
    let path = PathBuf::from(key);
    if !path.exists() {
        return Err(StoreError::NotFound(key.to_string()));
    }
    Ok(std::fs::read(&path)?)
}

pub fn save(key: &str, data: &[u8]) -> Result<(), StoreError> {
    let path = PathBuf::from(key);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(std::fs::write(&path, data)?)
}

pub fn remove(key: &str) -> Result<bool, StoreError> {
    let path = PathBuf::from(key);
    if path.exists() {
        std::fs::remove_file(&path)?;
        return Ok(true);
    }
    Ok(false)
}
