use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod catalog;
mod fetch;
mod mdx;
mod split;

#[derive(Parser)]
#[command(about = "Build-time tooling for the MCP docs server")]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Fetch and process all doc sources defined in the manifest
    FetchDocs {
        #[arg(long, default_value = "servers-manifest.json")]
        manifest: PathBuf,
        #[arg(long, default_value = "/docs")]
        docs_dir: PathBuf,
        #[arg(long, default_value = "/local-docs")]
        local_docs: PathBuf,
        #[arg(long, default_value = "/tmp/fetch")]
        tmp: PathBuf,
    },
    /// Convert .mdx files in-place to clean .md via AST transform
    StripMdx { docs_dir: PathBuf },
    /// Generate Tailwind CSS v4 class catalog
    GenerateCatalog { output: PathBuf },
    /// Split a single markdown file into one .md per heading
    Split { input: PathBuf, output_dir: PathBuf },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::FetchDocs {
            manifest,
            docs_dir,
            local_docs,
            tmp,
        } => fetch::run(&manifest, &docs_dir, &local_docs, &tmp),
        Cmd::StripMdx { docs_dir } => mdx::strip_dir(&docs_dir),
        Cmd::GenerateCatalog { output } => catalog::write_catalog(&output),
        Cmd::Split { input, output_dir } => split::run(&input, &output_dir),
    }
}
