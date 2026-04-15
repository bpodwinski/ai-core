//! Single-file Markdown splitter — one `.md` per heading.

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::Path;

/// Split a single Markdown file into one `.md` file per `##` or `###` heading.
///
/// Each section is written to `output_dir/<slug>.md`, where `<slug>` is the
/// heading title converted to lowercase kebab-case. Content before the first
/// heading is written as `overview.md`.
///
/// # Errors
///
/// Returns an error if `input` cannot be read or if any output file cannot be
/// written.
pub fn run(input: &Path, output_dir: &Path) -> Result<()> {
    let raw =
        std::fs::read_to_string(input).with_context(|| format!("reading {}", input.display()))?;
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("creating {}", output_dir.display()))?;

    let mut current_slug: Option<String> = Some("overview".to_string());
    let mut current_lines: Vec<&str> = Vec::new();
    let mut in_preamble = true;
    let mut seen: HashMap<String, usize> = HashMap::new();

    for line in raw.split('\n') {
        let h3 = line.strip_prefix("### ");
        let h2 = line.strip_prefix("## ");

        if let Some(title) = h3.or(h2) {
            flush_section(&current_slug, &current_lines, output_dir, &mut seen)?;
            current_slug = Some(to_slug(title));
            current_lines = vec![line];
            in_preamble = false;
        } else {
            if in_preamble && current_slug.is_none() {
                current_slug = Some("overview".to_string());
                current_lines = Vec::new();
            }
            current_lines.push(line);
        }
    }

    flush_section(&current_slug, &current_lines, output_dir, &mut seen)?;
    println!("Done.");
    Ok(())
}

fn flush_section(
    slug: &Option<String>,
    lines: &[&str],
    output_dir: &Path,
    seen: &mut HashMap<String, usize>,
) -> Result<()> {
    let Some(base_slug) = slug else {
        return Ok(());
    };
    let content = lines.join("\n");
    if content.trim().is_empty() {
        return Ok(());
    }
    // Deduplicate: first occurrence keeps the base slug; subsequent ones get -2, -3, ...
    let count = seen.entry(base_slug.clone()).or_insert(0);
    *count += 1;
    let unique_slug = if *count == 1 {
        base_slug.clone()
    } else {
        format!("{}-{}", base_slug, count)
    };
    let path = output_dir.join(format!("{}.md", unique_slug));
    let trimmed = content.trim_end().to_string();
    std::fs::write(&path, format!("{}\n", trimmed))
        .with_context(|| format!("writing {}", path.display()))?;
    println!("  → {}.md", unique_slug);
    Ok(())
}

fn to_slug(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut prev_dash = true;
    for c in title.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash {
            out.push('-');
            prev_dash = true;
        }
    }
    out.trim_matches('-').to_string()
}
