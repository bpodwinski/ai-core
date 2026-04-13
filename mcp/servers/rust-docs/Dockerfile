# ── Stage 1 : Rust ────────────────────────────────────────────────────────────
# Build context is mcp/ (parent directory)
FROM rust:1.94-slim AS rust-builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY servers/rust-docs/Cargo.toml servers/rust-docs/Cargo.lock ./
COPY servers/rust-docs/src/ src/

RUN cargo build --release

# ── Stage 2 : Fetch & process docs (config-driven from manifest) ─────────────
FROM node:20-slim AS docs-builder

RUN apt-get update && apt-get install -y git curl && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# npm deps (cached layer)
COPY servers/rust-docs/package.json ./
RUN npm install --silent

# Manifest (source of truth) + processing scripts
COPY servers-manifest.json ./
COPY servers/rust-docs/fetch-docs.mjs ./
COPY servers/rust-docs/strip-mdx.mjs ./
COPY servers/rust-docs/generate-catalog.mjs ./
COPY servers/rust-docs/split-daisyui.mjs ./

# Local docs (referenced by type:"local" entries in manifest)
COPY servers/rust-docs/local-docs/ /local-docs/

# Fetch all doc sources defined in the manifest
RUN node fetch-docs.mjs

# ── Stage 3 : Runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates wget && rm -rf /var/lib/apt/lists/*

COPY --from=rust-builder /app/target/release/mcp-rust-docs /usr/local/bin/
COPY --from=docs-builder /docs/ /docs/

EXPOSE 80
ENV DOCS_PATH=/docs
CMD ["mcp-rust-docs"]
