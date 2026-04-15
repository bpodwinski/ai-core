// Example 02 — AFTER: error enum and fallible functions fully documented.
// Note the # Errors section on each fallible fn, and per-variant docs on the enum.

use std::path::PathBuf;

/// Errors that can occur when reading or writing the key-value store.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// No entry exists for the requested key.
    #[error("not found: {0}")]
    NotFound(String),

    /// The process lacks permission to access the file at `path`.
    #[error("permission denied: {path}")]
    PermissionDenied { path: PathBuf },

    /// The stored bytes could not be deserialized from JSON.
    #[error("serialization failed: {0}")]
    Serialization(#[from] serde_json::Error),

    /// An underlying I/O error occurred.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Load the raw bytes stored under `key`.
///
/// # Errors
///
/// Returns [`StoreError::NotFound`] if `key` has no associated file.
/// Returns [`StoreError::Io`] on any other filesystem error.
pub fn load(key: &str) -> Result<Vec<u8>, StoreError> {
    let path = PathBuf::from(key);
    if !path.exists() {
        return Err(StoreError::NotFound(key.to_string()));
    }
    Ok(std::fs::read(&path)?)
}

/// Persist `data` under `key`, creating any missing parent directories.
///
/// # Errors
///
/// Returns [`StoreError::Io`] if the directory cannot be created or the file
/// cannot be written.
pub fn save(key: &str, data: &[u8]) -> Result<(), StoreError> {
    let path = PathBuf::from(key);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    Ok(std::fs::write(&path, data)?)
}

/// Remove the entry for `key` and return whether an entry existed.
///
/// Returns `Ok(true)` when a file was deleted, `Ok(false)` when the key was
/// already absent.
///
/// # Errors
///
/// Returns [`StoreError::Io`] if the file exists but cannot be deleted.
pub fn remove(key: &str) -> Result<bool, StoreError> {
    let path = PathBuf::from(key);
    if path.exists() {
        std::fs::remove_file(&path)?;
        return Ok(true);
    }
    Ok(false)
}
