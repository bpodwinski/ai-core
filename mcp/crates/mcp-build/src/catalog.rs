//! Tailwind CSS v4 class catalog generation.

use anyhow::{Context, Result};
use std::path::Path;

const CATALOG: &str = include_str!("tailwind-catalog.md");

/// Write the embedded Tailwind CSS v4 class catalog to `output`.
///
/// Creates any missing parent directories before writing.
///
/// # Errors
///
/// Returns an error if the parent directory cannot be created or the file
/// cannot be written.
pub fn write_catalog(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(output, CATALOG).with_context(|| format!("writing {}", output.display()))?;
    let lines = CATALOG.lines().count();
    println!("catalog.md écrit : {} ({} lignes)", output.display(), lines);
    Ok(())
}
