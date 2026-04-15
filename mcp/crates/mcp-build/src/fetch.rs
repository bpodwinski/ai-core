//! Documentation source fetching and transform pipeline.

use anyhow::{bail, Context, Result};
use mcp_common::{DocSource, Manifest, Transform};
use std::path::Path;
use std::process::Command;

/// Fetch and process all documentation sources declared in `manifest`.
///
/// For each `docSources` entry:
/// - `Git` sources are shallow-cloned into `tmp`, then copied to `docs_dir/<name>/`.
/// - `Url` sources are downloaded into `tmp`, then copied or transformed.
/// - `Local` sources are copied from `local_docs/<path>` to `docs_dir/<name>/`.
///
/// Post-processing [`Transform`]s are applied in declaration order after fetching.
///
/// # Errors
///
/// Returns an error if a source cannot be fetched, a directory cannot be
/// created, or a transform fails.
pub fn run(manifest: &Path, docs_dir: &Path, local_docs: &Path, tmp: &Path) -> Result<()> {
    let manifest = Manifest::load(manifest)?;
    manifest.validate()?;

    std::fs::create_dir_all(docs_dir)
        .with_context(|| format!("creating {}", docs_dir.display()))?;
    std::fs::create_dir_all(tmp).with_context(|| format!("creating {}", tmp.display()))?;

    let mut total = 0usize;
    for entry in &manifest.doc_sources {
        let Some(src) = &entry.source else {
            println!("-- {}: no source defined, skipping", entry.name);
            continue;
        };

        let out_dir = docs_dir.join(&entry.name);
        println!(
            "\n=> {} ({})",
            entry.name,
            match src {
                DocSource::Git { .. } => "git",
                DocSource::Url { .. } => "url",
                DocSource::Local { .. } => "local",
            }
        );

        match src {
            DocSource::Git { url, docs_path, .. } => {
                let clone_dir = tmp.join(&entry.name);
                if clone_dir.exists() {
                    fs_extra::dir::remove(&clone_dir).ok();
                }
                with_retry(&format!("git clone {url}"), || {
                    run_cmd(
                        Command::new("git")
                            .args(["clone", "--depth", "1", url])
                            .arg(&clone_dir),
                    )
                })?;
                let src_dir = clone_dir.join(docs_path);
                if out_dir.exists() {
                    fs_extra::dir::remove(&out_dir).ok();
                }
                copy_dir(&src_dir, &out_dir)?;
            }
            DocSource::Url { url, transforms } => {
                std::fs::create_dir_all(&out_dir)?;
                let tmp_file = tmp.join(format!("{}.md", entry.name));
                with_retry(&format!("download {url}"), || download_url(url, &tmp_file))?;
                if transforms.is_empty() {
                    std::fs::copy(&tmp_file, out_dir.join(format!("{}.md", entry.name)))?;
                }
            }
            DocSource::Local { path, .. } => {
                let local_path = local_docs.join(path);
                if !local_path.exists() {
                    bail!("Local docs not found: {}", local_path.display());
                }
                if out_dir.exists() {
                    fs_extra::dir::remove(&out_dir).ok();
                }
                copy_dir(&local_path, &out_dir)?;
            }
        }

        // Apply transforms
        for transform in src.transforms() {
            println!("   transform: {}", transform_name(*transform));
            match transform {
                Transform::StripMdx => {
                    crate::mdx::strip_dir(&out_dir)?;
                }
                Transform::GenerateCatalog => {
                    crate::catalog::write_catalog(&out_dir.join("catalog.md"))?;
                }
                Transform::Split => {
                    let tmp_file = tmp.join(format!("{}.md", entry.name));
                    crate::split::run(&tmp_file, &out_dir)?;
                }
            }
        }

        total += 1;
        println!("   OK {}", entry.name);
    }

    println!(
        "\nDone: {} doc sources fetched to {}",
        total,
        docs_dir.display()
    );
    Ok(())
}

fn transform_name(t: Transform) -> &'static str {
    match t {
        Transform::StripMdx => "strip-mdx",
        Transform::GenerateCatalog => "generate-catalog",
        Transform::Split => "split",
    }
}

fn run_cmd(cmd: &mut Command) -> Result<()> {
    let status = cmd
        .status()
        .with_context(|| format!("spawning {:?}", cmd))?;
    if !status.success() {
        bail!("{:?} failed with status {:?}", cmd, status);
    }
    Ok(())
}

/// Retry `f` up to 3 times with exponential back-off (2 s, 8 s between attempts).
fn with_retry<T, F: Fn() -> Result<T>>(label: &str, f: F) -> Result<T> {
    let delays_secs = [2u64, 8];
    let mut last_err: Option<anyhow::Error> = None;
    for attempt in 0..3usize {
        match f() {
            Ok(v) => return Ok(v),
            Err(e) => {
                if attempt < delays_secs.len() {
                    eprintln!(
                        "  [retry {}/3] {label} failed: {e} — retrying in {}s",
                        attempt + 1,
                        delays_secs[attempt]
                    );
                    std::thread::sleep(std::time::Duration::from_secs(delays_secs[attempt]));
                }
                last_err = Some(e);
            }
        }
    }
    Err(last_err.expect("loop runs at least once"))
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    let mut opts = fs_extra::dir::CopyOptions::new();
    opts.copy_inside = true;
    opts.content_only = true;
    opts.overwrite = true;
    std::fs::create_dir_all(dst)?;
    fs_extra::dir::copy(src, dst, &opts)
        .with_context(|| format!("copying {} → {}", src.display(), dst.display()))?;
    Ok(())
}

fn download_url(url: &str, out: &Path) -> Result<()> {
    let body = reqwest::blocking::get(url)
        .with_context(|| format!("GET {}", url))?
        .error_for_status()?
        .bytes()?;
    if let Some(parent) = out.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let path_ref: &Path = out;
    let _ = path_ref;
    std::fs::write(out, &body).with_context(|| format!("writing {}", out.display()))?;
    Ok(())
}
