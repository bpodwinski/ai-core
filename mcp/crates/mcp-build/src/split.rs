use anyhow::{Context, Result};
use std::path::Path;

pub fn run(input: &Path, output_dir: &Path) -> Result<()> {
    let raw =
        std::fs::read_to_string(input).with_context(|| format!("reading {}", input.display()))?;
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("creating {}", output_dir.display()))?;

    let mut current_slug: Option<String> = Some("overview".to_string());
    let mut current_lines: Vec<&str> = Vec::new();
    let mut in_preamble = true;

    for line in raw.split('\n') {
        let h3 = line.strip_prefix("### ");
        let h2 = line.strip_prefix("## ");

        if let Some(title) = h3.or(h2) {
            flush_section(&current_slug, &current_lines, output_dir)?;
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

    flush_section(&current_slug, &current_lines, output_dir)?;
    println!("Done.");
    Ok(())
}

fn flush_section(slug: &Option<String>, lines: &[&str], output_dir: &Path) -> Result<()> {
    let Some(slug) = slug else { return Ok(()) };
    let content = lines.join("\n");
    if content.trim().is_empty() {
        return Ok(());
    }
    let path = output_dir.join(format!("{}.md", slug));
    let trimmed = content.trim_end().to_string();
    std::fs::write(&path, format!("{}\n", trimmed))
        .with_context(|| format!("writing {}", path.display()))?;
    println!("  → {}.md", slug);
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
